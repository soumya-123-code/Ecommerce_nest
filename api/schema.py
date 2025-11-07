"""
GraphQL Schema for Django eCommerce Platform
This schema provides a complete GraphQL API for all models
"""

import graphene
from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from django.contrib.auth.models import User
from accounts.models import Profile, BankAccount, SocialLink
from categories.models import SuperCategory, MainCategory, SubCategory, MiniCategory
from products.models import Product, ProductImage, ProductRating, ProductSize
from orders.models import Order, OrderDetails, Coupon, Payment, Country, OrderSupplier, OrderDetailsSupplier
from payments.models import VendorPayments
from blog.models import Post, Comment
from newsletters.models import Newsletter
from contact.models import MessagesList
from settings.models import SocailLinks, ContactInfo, SupportNumber, SiteSetting, HomePageTheme
from home.models import (
    Carousel, HomeAdSidebar, HomeAdMiddlebar, HomeAdSupplier,
    HomeAdDaily, HomeAdDealTime, VendorDetailsAdImage, ShopAdSidebar,
    HotDealAd, HeadTextAd
)


# ==================== USER & AUTH TYPES ====================

class UserType(DjangoObjectType):
    class Meta:
        model = User
        fields = ('id', 'username', 'email', 'first_name', 'last_name', 'is_active', 'is_staff', 'is_superuser', 'date_joined')


class ProfileType(DjangoObjectType):
    class Meta:
        model = Profile
        fields = '__all__'


class BankAccountType(DjangoObjectType):
    class Meta:
        model = BankAccount
        fields = '__all__'


class SocialLinkType(DjangoObjectType):
    class Meta:
        model = SocialLink
        fields = '__all__'


# ==================== CATEGORY TYPES ====================

class SuperCategoryType(DjangoObjectType):
    class Meta:
        model = SuperCategory
        fields = '__all__'


class MainCategoryType(DjangoObjectType):
    class Meta:
        model = MainCategory
        fields = '__all__'


class SubCategoryType(DjangoObjectType):
    class Meta:
        model = SubCategory
        fields = '__all__'


class MiniCategoryType(DjangoObjectType):
    class Meta:
        model = MiniCategory
        fields = '__all__'


# ==================== PRODUCT TYPES ====================

class ProductType(DjangoObjectType):
    class Meta:
        model = Product
        fields = '__all__'


class ProductImageType(DjangoObjectType):
    class Meta:
        model = ProductImage
        fields = '__all__'


class ProductRatingType(DjangoObjectType):
    class Meta:
        model = ProductRating
        fields = '__all__'


class ProductSizeType(DjangoObjectType):
    class Meta:
        model = ProductSize
        fields = '__all__'


# ==================== ORDER TYPES ====================

class OrderType(DjangoObjectType):
    class Meta:
        model = Order
        fields = '__all__'


class OrderDetailsType(DjangoObjectType):
    class Meta:
        model = OrderDetails
        fields = '__all__'


class OrderSupplierType(DjangoObjectType):
    class Meta:
        model = OrderSupplier
        fields = '__all__'


class OrderDetailsSupplierType(DjangoObjectType):
    class Meta:
        model = OrderDetailsSupplier
        fields = '__all__'


class CouponType(DjangoObjectType):
    class Meta:
        model = Coupon
        fields = '__all__'


class PaymentType(DjangoObjectType):
    class Meta:
        model = Payment
        fields = '__all__'


class CountryType(DjangoObjectType):
    class Meta:
        model = Country
        fields = '__all__'


# ==================== PAYMENT TYPES ====================

class VendorPaymentType(DjangoObjectType):
    class Meta:
        model = VendorPayments
        fields = '__all__'


# ==================== BLOG TYPES ====================

class PostType(DjangoObjectType):
    class Meta:
        model = Post
        fields = '__all__'


class CommentType(DjangoObjectType):
    class Meta:
        model = Comment
        fields = '__all__'


# ==================== NEWSLETTER & CONTACT TYPES ====================

class NewsletterType(DjangoObjectType):
    class Meta:
        model = Newsletter
        fields = '__all__'


class ContactType(DjangoObjectType):
    class Meta:
        model = MessagesList
        fields = '__all__'


# ==================== SETTINGS TYPES ====================

class SiteSocialLinksType(DjangoObjectType):
    class Meta:
        model = SocailLinks
        fields = '__all__'


class ContactInfoType(DjangoObjectType):
    class Meta:
        model = ContactInfo
        fields = '__all__'


class SupportNumberType(DjangoObjectType):
    class Meta:
        model = SupportNumber
        fields = '__all__'


class SiteSettingType(DjangoObjectType):
    class Meta:
        model = SiteSetting
        fields = '__all__'


class HomePageThemeType(DjangoObjectType):
    class Meta:
        model = HomePageTheme
        fields = '__all__'


# ==================== HOME/AD TYPES ====================

class CarouselType(DjangoObjectType):
    class Meta:
        model = Carousel
        fields = '__all__'


class HomeAdSidebarType(DjangoObjectType):
    class Meta:
        model = HomeAdSidebar
        fields = '__all__'


class HomeAdMiddlebarType(DjangoObjectType):
    class Meta:
        model = HomeAdMiddlebar
        fields = '__all__'


class HomeAdSupplierType(DjangoObjectType):
    class Meta:
        model = HomeAdSupplier
        fields = '__all__'


class HomeAdDailyType(DjangoObjectType):
    class Meta:
        model = HomeAdDaily
        fields = '__all__'


class HomeAdDealTimeType(DjangoObjectType):
    class Meta:
        model = HomeAdDealTime
        fields = '__all__'


class VendorDetailsAdImageType(DjangoObjectType):
    class Meta:
        model = VendorDetailsAdImage
        fields = '__all__'


class ShopAdSidebarType(DjangoObjectType):
    class Meta:
        model = ShopAdSidebar
        fields = '__all__'


class HotDealAdType(DjangoObjectType):
    class Meta:
        model = HotDealAd
        fields = '__all__'


class HeadTextAdType(DjangoObjectType):
    class Meta:
        model = HeadTextAd
        fields = '__all__'


# ==================== QUERIES ====================

class Query(graphene.ObjectType):
    # User & Auth Queries
    all_users = graphene.List(UserType)
    user = graphene.Field(UserType, id=graphene.Int())
    me = graphene.Field(UserType)
    all_profiles = graphene.List(ProfileType)
    profile = graphene.Field(ProfileType, id=graphene.Int())
    profile_by_slug = graphene.Field(ProfileType, slug=graphene.String())

    # Category Queries
    all_super_categories = graphene.List(SuperCategoryType)
    super_category = graphene.Field(SuperCategoryType, id=graphene.Int())
    all_main_categories = graphene.List(MainCategoryType)
    main_category = graphene.Field(MainCategoryType, id=graphene.Int())
    all_sub_categories = graphene.List(SubCategoryType)
    sub_category = graphene.Field(SubCategoryType, id=graphene.Int())
    all_mini_categories = graphene.List(MiniCategoryType)
    mini_category = graphene.Field(MiniCategoryType, id=graphene.Int())

    # Product Queries
    all_products = graphene.List(ProductType, limit=graphene.Int(), offset=graphene.Int())
    product = graphene.Field(ProductType, id=graphene.Int())
    product_by_slug = graphene.Field(ProductType, slug=graphene.String())
    products_by_vendor = graphene.List(ProductType, vendor_id=graphene.Int())
    products_by_category = graphene.List(ProductType, category_id=graphene.Int())

    # Order Queries
    all_orders = graphene.List(OrderType)
    order = graphene.Field(OrderType, id=graphene.Int())
    orders_by_user = graphene.List(OrderType, user_id=graphene.Int())

    # Blog Queries
    all_posts = graphene.List(PostType)
    post = graphene.Field(PostType, id=graphene.Int())
    post_by_slug = graphene.Field(PostType, slug=graphene.String())

    # Newsletter & Contact
    all_newsletters = graphene.List(NewsletterType)
    all_contact_messages = graphene.List(ContactType)

    # Settings Queries
    site_settings = graphene.Field(SiteSettingType)
    contact_info = graphene.List(ContactInfoType)

    # Home/Ads Queries
    all_carousels = graphene.List(CarouselType)
    hot_deals = graphene.List(HotDealAdType)

    # Resolvers - Users
    def resolve_all_users(self, info):
        return User.objects.all()

    def resolve_user(self, info, id):
        return User.objects.get(pk=id)

    def resolve_me(self, info):
        user = info.context.user
        if user.is_anonymous:
            raise Exception('Not logged in!')
        return user

    def resolve_all_profiles(self, info):
        return Profile.objects.all()

    def resolve_profile(self, info, id):
        return Profile.objects.get(pk=id)

    def resolve_profile_by_slug(self, info, slug):
        return Profile.objects.get(slug=slug)

    # Resolvers - Categories
    def resolve_all_super_categories(self, info):
        return SuperCategory.objects.all()

    def resolve_super_category(self, info, id):
        return SuperCategory.objects.get(pk=id)

    def resolve_all_main_categories(self, info):
        return MainCategory.objects.all()

    def resolve_main_category(self, info, id):
        return MainCategory.objects.get(pk=id)

    def resolve_all_sub_categories(self, info):
        return SubCategory.objects.all()

    def resolve_sub_category(self, info, id):
        return SubCategory.objects.get(pk=id)

    def resolve_all_mini_categories(self, info):
        return MiniCategory.objects.all()

    def resolve_mini_category(self, info, id):
        return MiniCategory.objects.get(pk=id)

    # Resolvers - Products
    def resolve_all_products(self, info, limit=20, offset=0):
        return Product.objects.filter(PRDISactive=True, PRDISDeleted=False)[offset:offset+limit]

    def resolve_product(self, info, id):
        return Product.objects.get(pk=id)

    def resolve_product_by_slug(self, info, slug):
        return Product.objects.get(PRDSlug=slug)

    def resolve_products_by_vendor(self, info, vendor_id):
        return Product.objects.filter(product_vendor_id=vendor_id, PRDISactive=True, PRDISDeleted=False)

    def resolve_products_by_category(self, info, category_id):
        return Product.objects.filter(product_minicategor_id=category_id, PRDISactive=True, PRDISDeleted=False)

    # Resolvers - Orders
    def resolve_all_orders(self, info):
        return Order.objects.all()

    def resolve_order(self, info, id):
        return Order.objects.get(pk=id)

    def resolve_orders_by_user(self, info, user_id):
        return Order.objects.filter(user_id=user_id)

    # Resolvers - Blog
    def resolve_all_posts(self, info):
        return Post.objects.all().order_by('-post_date')

    def resolve_post(self, info, id):
        return Post.objects.get(pk=id)

    def resolve_post_by_slug(self, info, slug):
        return Post.objects.get(post_Slug=slug)

    # Resolvers - Newsletter & Contact
    def resolve_all_newsletters(self, info):
        return Newsletter.objects.all()

    def resolve_all_contact_messages(self, info):
        return MessagesList.objects.all()

    # Resolvers - Settings
    def resolve_site_settings(self, info):
        return SiteSetting.objects.first()

    def resolve_contact_info(self, info):
        return ContactInfo.objects.filter(active=True)

    # Resolvers - Home/Ads
    def resolve_all_carousels(self, info):
        return Carousel.objects.all()

    def resolve_hot_deals(self, info):
        return HotDealAd.objects.all()


# ==================== MUTATIONS ====================

class CreateProduct(graphene.Mutation):
    class Arguments:
        product_name = graphene.String(required=True)
        description = graphene.String(required=True)
        price = graphene.Float(required=True)
        sku = graphene.String()

    product = graphene.Field(ProductType)

    def mutate(self, info, product_name, description, price, sku=None):
        user = info.context.user
        if user.is_anonymous:
            raise Exception('You must be logged in!')

        product = Product.objects.create(
            product_vendor=user.profile,
            product_name=product_name,
            product_description=description,
            PRDPrice=price,
            PRDSKU=sku or ''
        )
        return CreateProduct(product=product)


class UpdateProduct(graphene.Mutation):
    class Arguments:
        id = graphene.Int(required=True)
        product_name = graphene.String()
        description = graphene.String()
        price = graphene.Float()

    product = graphene.Field(ProductType)

    def mutate(self, info, id, product_name=None, description=None, price=None):
        product = Product.objects.get(pk=id)

        if product_name:
            product.product_name = product_name
        if description:
            product.product_description = description
        if price:
            product.PRDPrice = price

        product.save()
        return UpdateProduct(product=product)


class DeleteProduct(graphene.Mutation):
    class Arguments:
        id = graphene.Int(required=True)

    success = graphene.Boolean()

    def mutate(self, info, id):
        product = Product.objects.get(pk=id)
        product.PRDISDeleted = True
        product.save()
        return DeleteProduct(success=True)


class CreateOrder(graphene.Mutation):
    class Arguments:
        email_client = graphene.String(required=True)
        amount = graphene.String(required=True)

    order = graphene.Field(OrderType)

    def mutate(self, info, email_client, amount):
        user = info.context.user
        order = Order.objects.create(
            user=user if not user.is_anonymous else None,
            email_client=email_client,
            amount=amount
        )
        return CreateOrder(order=order)


class SubscribeNewsletter(graphene.Mutation):
    class Arguments:
        email = graphene.String(required=True)

    newsletter = graphene.Field(NewsletterType)

    def mutate(self, info, email):
        newsletter, created = Newsletter.objects.get_or_create(email=email)
        return SubscribeNewsletter(newsletter=newsletter)


class CreateContactMessage(graphene.Mutation):
    class Arguments:
        name = graphene.String(required=True)
        email = graphene.String(required=True)
        phone = graphene.String(required=True)
        subject = graphene.String(required=True)
        message = graphene.String(required=True)

    contact = graphene.Field(ContactType)

    def mutate(self, info, name, email, phone, subject, message):
        contact = MessagesList.objects.create(
            name=name,
            email=email,
            phone=phone,
            subject=subject,
            message=message
        )
        return CreateContactMessage(contact=contact)


class Mutation(graphene.ObjectType):
    create_product = CreateProduct.Field()
    update_product = UpdateProduct.Field()
    delete_product = DeleteProduct.Field()
    create_order = CreateOrder.Field()
    subscribe_newsletter = SubscribeNewsletter.Field()
    create_contact_message = CreateContactMessage.Field()


# ==================== SCHEMA ====================

schema = graphene.Schema(query=Query, mutation=Mutation)
