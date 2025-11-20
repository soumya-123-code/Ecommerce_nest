use chrono::Utc;
use diesel::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::db::DbConnection;
use crate::dto::CartItem;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::{AppError, AppResult};

pub struct OrderService;

impl OrderService {
    /// Create order from cart items
    /// Django: orders/views.py - checkout_view
    pub fn create_order(
        conn: &mut DbConnection,
        user_id: i32,
        items: &[CartItem],
        shipping_address: &str,
        billing_address: &str,
        phone: &str,
        email: &str,
        notes: Option<String>,
        coupon_code: Option<&str>,
    ) -> AppResult<Order> {
        // Validate and calculate totals
        let mut subtotal = Decimal::ZERO;
        let mut order_items: Vec<(Product, i32, Option<String>, Decimal)> = Vec::new();

        for item in items {
            let product = products::table
                .find(item.product_id)
                .first::<Product>(conn)
                .map_err(|_| AppError::NotFound(format!("Product {} not found", item.product_id)))?;

            if !product.is_active {
                return Err(AppError::BadRequest(format!("Product {} is not available", product.name)));
            }

            if product.stock < item.quantity {
                return Err(AppError::BadRequest(format!("Insufficient stock for {}", product.name)));
            }

            let price = product.discount_price.unwrap_or(product.price);
            let item_total = price * Decimal::from(item.quantity);
            subtotal += item_total;

            order_items.push((product, item.quantity, item.size.clone(), price));
        }

        // Apply coupon if provided
        let (discount, coupon_id) = if let Some(code) = coupon_code {
            Self::apply_coupon(conn, code, subtotal)?
        } else {
            (Decimal::ZERO, None)
        };

        // Calculate totals
        let shipping_cost = Self::calculate_shipping(subtotal);
        let tax = (subtotal - discount) * Decimal::new(10, 2); // 10% tax
        let total = subtotal + shipping_cost + tax - discount;

        // Generate order number
        let order_number = format!("ORD-{}", Uuid::new_v4().to_string()[..8].to_uppercase());

        // Create order
        let new_order = NewOrder {
            user_id,
            order_number: order_number.clone(),
            status: OrderStatus::Pending.as_str().to_string(),
            subtotal,
            shipping_cost,
            tax,
            discount,
            total,
            shipping_address: shipping_address.to_string(),
            billing_address: billing_address.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            notes,
            coupon_id,
        };

        diesel::insert_into(orders::table)
            .values(&new_order)
            .execute(conn)?;

        let order = orders::table
            .filter(orders::order_number.eq(&order_number))
            .first::<Order>(conn)?;

        // Create order details and split by vendor
        let mut vendor_totals: std::collections::HashMap<i32, Decimal> = std::collections::HashMap::new();
        let mut vendor_details: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();

        for (product, quantity, size, unit_price) in order_items {
            let total_price = unit_price * Decimal::from(quantity);

            let new_detail = NewOrderDetail {
                order_id: order.id,
                product_id: product.id,
                product_name: product.name.clone(),
                size,
                quantity,
                unit_price,
                total_price,
            };

            diesel::insert_into(order_details::table)
                .values(&new_detail)
                .execute(conn)?;

            let detail = order_details::table
                .order(order_details::id.desc())
                .first::<OrderDetail>(conn)?;

            // Update product stock
            diesel::update(products::table.find(product.id))
                .set(products::stock.eq(products::stock - quantity))
                .execute(conn)?;

            // Track vendor totals
            *vendor_totals.entry(product.vendor_id).or_insert(Decimal::ZERO) += total_price;
            vendor_details.entry(product.vendor_id)
                .or_default()
                .push(detail.id);
        }

        // Create OrderSupplier records for each vendor
        let commission_rate = Decimal::new(10, 2); // 10% commission

        for (vendor_id, vendor_subtotal) in vendor_totals {
            let commission_amount = vendor_subtotal * commission_rate;
            let payout_amount = vendor_subtotal - commission_amount;

            let new_supplier = NewOrderSupplier {
                order_id: order.id,
                vendor_id,
                status: OrderStatus::Pending.as_str().to_string(),
                subtotal: vendor_subtotal,
                commission_rate,
                commission_amount,
                payout_amount,
            };

            diesel::insert_into(order_suppliers::table)
                .values(&new_supplier)
                .execute(conn)?;

            let supplier = order_suppliers::table
                .order(order_suppliers::id.desc())
                .first::<OrderSupplier>(conn)?;

            // Link details to supplier
            if let Some(detail_ids) = vendor_details.get(&vendor_id) {
                for detail_id in detail_ids {
                    let link = NewOrderDetailSupplier {
                        order_supplier_id: supplier.id,
                        order_detail_id: *detail_id,
                    };
                    diesel::insert_into(order_details_suppliers::table)
                        .values(&link)
                        .execute(conn)?;
                }
            }
        }

        Ok(order)
    }

    /// Get user orders
    pub fn get_user_orders(conn: &mut DbConnection, user_id: i32) -> AppResult<Vec<Order>> {
        orders::table
            .filter(orders::user_id.eq(user_id))
            .order(orders::created_at.desc())
            .load::<Order>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get order by ID
    pub fn get_order(conn: &mut DbConnection, order_id: i32, user_id: i32) -> AppResult<Order> {
        orders::table
            .filter(orders::id.eq(order_id))
            .filter(orders::user_id.eq(user_id))
            .first::<Order>(conn)
            .map_err(|_| AppError::NotFound("Order not found".to_string()))
    }

    /// Get order details
    pub fn get_order_details(conn: &mut DbConnection, order_id: i32) -> AppResult<Vec<OrderDetail>> {
        order_details::table
            .filter(order_details::order_id.eq(order_id))
            .load::<OrderDetail>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get vendor orders
    /// Django: supplier_panel/views.py - supplier_orders
    pub fn get_vendor_orders(conn: &mut DbConnection, vendor_id: i32) -> AppResult<Vec<OrderSupplier>> {
        order_suppliers::table
            .filter(order_suppliers::vendor_id.eq(vendor_id))
            .order(order_suppliers::id.desc())
            .load::<OrderSupplier>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Update vendor order status
    /// Django: supplier_panel/views.py - supplier_order_update
    pub fn update_vendor_order(
        conn: &mut DbConnection,
        order_supplier_id: i32,
        vendor_id: i32,
        status: &str,
        tracking_number: Option<String>,
    ) -> AppResult<OrderSupplier> {
        let supplier = order_suppliers::table
            .find(order_supplier_id)
            .first::<OrderSupplier>(conn)?;

        if supplier.vendor_id != vendor_id {
            return Err(AppError::Forbidden("Not your order".to_string()));
        }

        let mut updates = UpdateOrderSupplier {
            status: Some(status.to_string()),
            tracking_number,
            shipped_at: None,
            delivered_at: None,
        };

        // Set timestamps based on status
        let now = Utc::now().naive_utc();
        if status == "shipped" {
            updates.shipped_at = Some(now);
        } else if status == "delivered" {
            updates.delivered_at = Some(now);

            // Process referral commission when delivered
            Self::process_referral_commission(conn, &supplier)?;
        }

        diesel::update(order_suppliers::table.find(order_supplier_id))
            .set(&updates)
            .execute(conn)?;

        order_suppliers::table
            .find(order_supplier_id)
            .first::<OrderSupplier>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Apply coupon and return discount
    fn apply_coupon(
        conn: &mut DbConnection,
        code: &str,
        subtotal: Decimal,
    ) -> AppResult<(Decimal, Option<i32>)> {
        let coupon = coupons::table
            .filter(coupons::code.eq(code))
            .filter(coupons::is_active.eq(true))
            .first::<Coupon>(conn)
            .optional()?
            .ok_or_else(|| AppError::Validation("Invalid coupon code".to_string()))?;

        let now = Utc::now().naive_utc();
        if coupon.valid_from > now || coupon.valid_to < now {
            return Err(AppError::Validation("Coupon has expired".to_string()));
        }

        if subtotal < coupon.min_purchase {
            return Err(AppError::Validation(format!(
                "Minimum purchase of {} required",
                coupon.min_purchase
            )));
        }

        if let Some(max_uses) = coupon.max_uses {
            if coupon.used_count >= max_uses {
                return Err(AppError::Validation("Coupon usage limit reached".to_string()));
            }
        }

        let discount = match coupon.discount_type.as_str() {
            "percentage" => subtotal * coupon.discount_value / Decimal::from(100),
            "fixed" => coupon.discount_value,
            _ => Decimal::ZERO,
        };

        // Increment used count
        diesel::update(coupons::table.find(coupon.id))
            .set(coupons::used_count.eq(coupons::used_count + 1))
            .execute(conn)?;

        Ok((discount, Some(coupon.id)))
    }

    /// Calculate shipping cost
    fn calculate_shipping(subtotal: Decimal) -> Decimal {
        if subtotal > Decimal::from(100) {
            Decimal::ZERO // Free shipping over $100
        } else {
            Decimal::new(10, 0) // $10 flat rate
        }
    }

    /// Process referral commission
    /// Django: orders/views.py - referral commission logic
    fn process_referral_commission(
        conn: &mut DbConnection,
        order_supplier: &OrderSupplier,
    ) -> AppResult<()> {
        // Get the order to find the customer
        let order = orders::table
            .find(order_supplier.order_id)
            .first::<Order>(conn)?;

        // Get customer's profile to find referrer
        let profile = profiles::table
            .filter(profiles::user_id.eq(order.user_id))
            .first::<Profile>(conn)?;

        if let Some(referrer_id) = profile.referred_by_id {
            // 2.5% referral commission
            let commission = order_supplier.payout_amount * Decimal::new(25, 3);

            // Update referrer's wallet
            diesel::update(profiles::table.find(referrer_id))
                .set(profiles::wallet_balance.eq(profiles::wallet_balance + commission))
                .execute(conn)?;
        }

        Ok(())
    }

    /// Create payment record
    pub fn create_payment(
        conn: &mut DbConnection,
        order_id: i32,
        payment_method: &str,
        transaction_id: Option<String>,
        amount: Decimal,
        currency: &str,
        status: &str,
    ) -> AppResult<Payment> {
        let new_payment = NewPayment {
            order_id,
            payment_method: payment_method.to_string(),
            transaction_id,
            amount,
            currency: currency.to_string(),
            status: status.to_string(),
            gateway_response: None,
        };

        diesel::insert_into(payments::table)
            .values(&new_payment)
            .execute(conn)?;

        payments::table
            .order(payments::id.desc())
            .first::<Payment>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Update payment status
    pub fn update_payment_status(
        conn: &mut DbConnection,
        payment_id: i32,
        status: &str,
        transaction_id: Option<String>,
    ) -> AppResult<()> {
        let updates = UpdatePayment {
            status: Some(status.to_string()),
            transaction_id,
            gateway_response: None,
        };

        diesel::update(payments::table.find(payment_id))
            .set(&updates)
            .execute(conn)?;

        Ok(())
    }
}
