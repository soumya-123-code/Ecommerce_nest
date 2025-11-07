import { NestFactory } from '@nestjs/core';
import { ValidationPipe } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import * as cookieParser from 'cookie-parser';
import * as compression from 'compression';
import helmet from 'helmet';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  const configService = app.get(ConfigService);
  const port = configService.get('PORT') || 4000;

  // Security middleware
  app.use(helmet({
    contentSecurityPolicy: process.env.NODE_ENV === 'production' ? undefined : false,
    crossOriginEmbedderPolicy: false,
  }));

  // Enable CORS
  app.enableCors({
    origin: [
      configService.get('FRONTEND_URL') || 'http://localhost:3000',
      configService.get('ADMIN_URL') || 'http://localhost:3001',
      configService.get('VENDOR_URL') || 'http://localhost:3002',
    ],
    credentials: true,
  });

  // Compression
  app.use(compression());

  // Cookie parser
  app.use(cookieParser());

  // Global validation pipe
  app.useGlobalPipes(
    new ValidationPipe({
      whitelist: true,
      forbidNonWhitelisted: true,
      transform: true,
      transformOptions: {
        enableImplicitConversion: true,
      },
    }),
  );

  // Global prefix
  app.setGlobalPrefix('api');

  await app.listen(port);
  console.log(`🚀 Application is running on: http://localhost:${port}/graphql`);
}

bootstrap();
