import { Injectable } from '@nestjs/common';
import { PrismaService } from '../../prisma/prisma.service';

@Injectable()
export class ProductsService {
  constructor(private prisma: PrismaService) {}

  async findAll(skip = 0, take = 20) {
    return this.prisma.product.findMany({
      skip,
      take,
      where: { isDeleted: false, isActive: true },
      include: {
        vendor: true,
        images: true,
        sizes: true,
        superCategory: true,
        mainCategory: true,
        subCategory: true,
        miniCategory: true,
      },
      orderBy: { createdAt: 'desc' },
    });
  }

  async findOne(id: string) {
    return this.prisma.product.findUnique({
      where: { id },
      include: {
        vendor: true,
        images: true,
        sizes: true,
        ratings: { include: { customer: true } },
        superCategory: true,
        mainCategory: true,
        subCategory: true,
        miniCategory: true,
      },
    });
  }

  async findByVendor(vendorId: string) {
    return this.prisma.product.findMany({
      where: { vendorId, isDeleted: false },
      include: { images: true, sizes: true },
    });
  }
}
