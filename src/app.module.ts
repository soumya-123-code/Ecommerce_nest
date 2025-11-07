import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { GraphQLModule } from '@nestjs/graphql';
import { ApolloDriver, ApolloDriverConfig } from '@nestjs/apollo';
import { ThrottlerModule } from '@nestjs/throttler';
import { join } from 'path';
import { PrismaModule } from './prisma/prisma.module';
import { AuthModule } from './modules/auth/auth.module';
import { UsersModule } from './modules/users/users.module';
import { ProductsModule } from './modules/products/products.module';
import { CategoriesModule } from './modules/categories/categories.module';
import { OrdersModule } from './modules/orders/orders.module';
import { PaymentsModule } from './modules/payments/payments.module';
import { VendorsModule } from './modules/vendors/vendors.module';
import { BlogModule } from './modules/blog/blog.module';
import { NewsletterModule } from './modules/newsletter/newsletter.module';
import { ContactModule } from './modules/contact/contact.module';
import { SettingsModule } from './modules/settings/settings.module';

@Module({
  imports: [
    // Configuration
    ConfigModule.forRoot({
      isGlobal: true,
      envFilePath: '.env',
    }),

    // GraphQL
    GraphQLModule.forRoot<ApolloDriverConfig>({
      driver: ApolloDriver,
      autoSchemaFile: join(process.cwd(), 'src/schema.gql'),
      sortSchema: true,
      playground: true,
      introspection: true,
      context: ({ req, res }) => ({ req, res }),
      formatError: (error) => {
        return {
          message: error.message,
          code: error.extensions?.code,
          locations: error.locations,
          path: error.path,
        };
      },
    }),

    // Rate limiting
    ThrottlerModule.forRoot([{
      ttl: 60000,
      limit: 100,
    }]),

    // Database
    PrismaModule,

    // Feature modules
    AuthModule,
    UsersModule,
    ProductsModule,
    CategoriesModule,
    OrdersModule,
    PaymentsModule,
    VendorsModule,
    BlogModule,
    NewsletterModule,
    ContactModule,
    SettingsModule,
  ],
})
export class AppModule {}
