// @generated automatically by Diesel CLI.
// This represents the database schema converted from Django models

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        is_active -> Bool,
        is_staff -> Bool,
        is_superuser -> Bool,
        date_joined -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Integer,
        user_id -> Integer,
        phone -> Nullable<Varchar>,
        address -> Nullable<Text>,
        city -> Nullable<Varchar>,
        country_id -> Nullable<Integer>,
        postal_code -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        is_vendor -> Bool,
        vendor_admission -> Bool,
        wallet_balance -> Decimal,
        referral_code -> Nullable<Varchar>,
        referred_by_id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    bank_accounts (id) {
        id -> Integer,
        profile_id -> Integer,
        bank_name -> Varchar,
        account_name -> Varchar,
        account_number -> Varchar,
        routing_number -> Nullable<Varchar>,
        swift_code -> Nullable<Varchar>,
        is_default -> Bool,
    }
}

diesel::table! {
    countries (id) {
        id -> Integer,
        name -> Varchar,
        code -> Varchar,
        currency_code -> Varchar,
        currency_symbol -> Varchar,
    }
}

diesel::table! {
    super_categories (id) {
        id -> Integer,
        name -> Varchar,
        slug -> Varchar,
        image -> Nullable<Varchar>,
        is_active -> Bool,
    }
}

diesel::table! {
    main_categories (id) {
        id -> Integer,
        super_category_id -> Integer,
        name -> Varchar,
        slug -> Varchar,
        image -> Nullable<Varchar>,
        is_active -> Bool,
    }
}

diesel::table! {
    sub_categories (id) {
        id -> Integer,
        main_category_id -> Integer,
        name -> Varchar,
        slug -> Varchar,
        image -> Nullable<Varchar>,
        is_active -> Bool,
    }
}

diesel::table! {
    mini_categories (id) {
        id -> Integer,
        sub_category_id -> Integer,
        name -> Varchar,
        slug -> Varchar,
        image -> Nullable<Varchar>,
        is_active -> Bool,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        vendor_id -> Integer,
        mini_category_id -> Integer,
        name -> Varchar,
        slug -> Varchar,
        description -> Text,
        price -> Decimal,
        discount_price -> Nullable<Decimal>,
        stock -> Integer,
        sku -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        is_active -> Bool,
        is_featured -> Bool,
        views_count -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    product_images (id) {
        id -> Integer,
        product_id -> Integer,
        image -> Varchar,
        alt_text -> Nullable<Varchar>,
        sort_order -> Integer,
    }
}

diesel::table! {
    product_sizes (id) {
        id -> Integer,
        product_id -> Integer,
        size -> Varchar,
        stock -> Integer,
        price_adjustment -> Decimal,
    }
}

diesel::table! {
    product_ratings (id) {
        id -> Integer,
        product_id -> Integer,
        user_id -> Integer,
        rating -> Integer,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        user_id -> Integer,
        order_number -> Varchar,
        status -> Varchar,
        subtotal -> Decimal,
        shipping_cost -> Decimal,
        tax -> Decimal,
        discount -> Decimal,
        total -> Decimal,
        shipping_address -> Text,
        billing_address -> Text,
        phone -> Varchar,
        email -> Varchar,
        notes -> Nullable<Text>,
        coupon_id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    order_details (id) {
        id -> Integer,
        order_id -> Integer,
        product_id -> Integer,
        product_name -> Varchar,
        size -> Nullable<Varchar>,
        quantity -> Integer,
        unit_price -> Decimal,
        total_price -> Decimal,
    }
}

diesel::table! {
    order_suppliers (id) {
        id -> Integer,
        order_id -> Integer,
        vendor_id -> Integer,
        status -> Varchar,
        subtotal -> Decimal,
        commission_rate -> Decimal,
        commission_amount -> Decimal,
        payout_amount -> Decimal,
        tracking_number -> Nullable<Varchar>,
        shipped_at -> Nullable<Timestamp>,
        delivered_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    order_details_suppliers (id) {
        id -> Integer,
        order_supplier_id -> Integer,
        order_detail_id -> Integer,
    }
}

diesel::table! {
    coupons (id) {
        id -> Integer,
        code -> Varchar,
        discount_type -> Varchar,
        discount_value -> Decimal,
        min_purchase -> Decimal,
        max_uses -> Nullable<Integer>,
        used_count -> Integer,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        is_active -> Bool,
    }
}

diesel::table! {
    payments (id) {
        id -> Integer,
        order_id -> Integer,
        payment_method -> Varchar,
        transaction_id -> Nullable<Varchar>,
        amount -> Decimal,
        currency -> Varchar,
        status -> Varchar,
        gateway_response -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    vendor_payments (id) {
        id -> Integer,
        vendor_id -> Integer,
        order_supplier_id -> Nullable<Integer>,
        amount -> Decimal,
        payment_type -> Varchar,
        status -> Varchar,
        reference_number -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        processed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        author_id -> Integer,
        title -> Varchar,
        slug -> Varchar,
        content -> Text,
        excerpt -> Nullable<Text>,
        featured_image -> Nullable<Varchar>,
        is_published -> Bool,
        views_count -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Integer,
        post_id -> Integer,
        user_id -> Integer,
        content -> Text,
        is_approved -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    post_views (id) {
        id -> Integer,
        post_id -> Integer,
        ip_address -> Varchar,
        user_agent -> Nullable<Text>,
        browser -> Nullable<Varchar>,
        os -> Nullable<Varchar>,
        device -> Nullable<Varchar>,
        is_mobile -> Bool,
        viewed_at -> Timestamp,
    }
}

diesel::table! {
    newsletters (id) {
        id -> Integer,
        email -> Varchar,
        is_active -> Bool,
        subscribed_at -> Timestamp,
    }
}

diesel::table! {
    contact_messages (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        subject -> Varchar,
        message -> Text,
        is_read -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    site_settings (id) {
        id -> Integer,
        site_name -> Varchar,
        site_logo -> Nullable<Varchar>,
        favicon -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        meta_keywords -> Nullable<Text>,
        footer_text -> Nullable<Text>,
        maintenance_mode -> Bool,
    }
}

diesel::table! {
    carousels (id) {
        id -> Integer,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        image -> Varchar,
        link -> Nullable<Varchar>,
        sort_order -> Integer,
        is_active -> Bool,
    }
}

diesel::table! {
    home_ads (id) {
        id -> Integer,
        ad_type -> Varchar,
        title -> Nullable<Varchar>,
        image -> Varchar,
        link -> Nullable<Varchar>,
        position -> Varchar,
        sort_order -> Integer,
        is_active -> Bool,
        starts_at -> Nullable<Timestamp>,
        ends_at -> Nullable<Timestamp>,
    }
}

// Define relationships
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(profiles -> countries (country_id));
diesel::joinable!(bank_accounts -> profiles (profile_id));
diesel::joinable!(main_categories -> super_categories (super_category_id));
diesel::joinable!(sub_categories -> main_categories (main_category_id));
diesel::joinable!(mini_categories -> sub_categories (sub_category_id));
diesel::joinable!(products -> profiles (vendor_id));
diesel::joinable!(products -> mini_categories (mini_category_id));
diesel::joinable!(product_images -> products (product_id));
diesel::joinable!(product_sizes -> products (product_id));
diesel::joinable!(product_ratings -> products (product_id));
diesel::joinable!(product_ratings -> users (user_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(orders -> coupons (coupon_id));
diesel::joinable!(order_details -> orders (order_id));
diesel::joinable!(order_details -> products (product_id));
diesel::joinable!(order_suppliers -> orders (order_id));
diesel::joinable!(order_suppliers -> profiles (vendor_id));
diesel::joinable!(order_details_suppliers -> order_suppliers (order_supplier_id));
diesel::joinable!(order_details_suppliers -> order_details (order_detail_id));
diesel::joinable!(payments -> orders (order_id));
diesel::joinable!(vendor_payments -> profiles (vendor_id));
diesel::joinable!(vendor_payments -> order_suppliers (order_supplier_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(post_views -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    profiles,
    bank_accounts,
    countries,
    super_categories,
    main_categories,
    sub_categories,
    mini_categories,
    products,
    product_images,
    product_sizes,
    product_ratings,
    orders,
    order_details,
    order_suppliers,
    order_details_suppliers,
    coupons,
    payments,
    vendor_payments,
    posts,
    comments,
    post_views,
    newsletters,
    contact_messages,
    site_settings,
    carousels,
    home_ads,
);
