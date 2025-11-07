import { Injectable, NotFoundException } from '@nestjs/common';
import { PrismaService } from '../../prisma/prisma.service';
import * as bcrypt from 'bcrypt';
import { UserRole } from '@prisma/client';

@Injectable()
export class UsersService {
  constructor(private prisma: PrismaService) {}

  async findById(id: string) {
    const user = await this.prisma.user.findUnique({
      where: { id },
      include: { profile: true },
    });

    if (!user) {
      throw new NotFoundException('User not found');
    }

    return user;
  }

  async findByEmail(email: string) {
    return this.prisma.user.findUnique({
      where: { email },
      include: { profile: true },
    });
  }

  async findByUsername(username: string) {
    return this.prisma.user.findUnique({
      where: { username },
      include: { profile: true },
    });
  }

  async findByEmailOrUsername(identifier: string) {
    const user = await this.prisma.user.findFirst({
      where: {
        OR: [{ email: identifier }, { username: identifier }],
      },
      include: { profile: true },
    });

    return user;
  }

  async create(data: {
    email: string;
    username: string;
    password: string;
    firstName?: string;
    lastName?: string;
    role?: UserRole;
  }) {
    const hashedPassword = await bcrypt.hash(data.password, 10);

    const user = await this.prisma.user.create({
      data: {
        email: data.email,
        username: data.username,
        password: hashedPassword,
        firstName: data.firstName,
        lastName: data.lastName,
        role: data.role || UserRole.CUSTOMER,
        profile: {
          create: {
            displayName: data.username,
            slug: await this.generateUniqueSlug(data.username),
            status: data.role === UserRole.VENDOR ? 'VENDOR' : 'CUSTOMER',
          },
        },
      },
      include: { profile: true },
    });

    return user;
  }

  async validatePassword(plainPassword: string, hashedPassword: string) {
    return bcrypt.compare(plainPassword, hashedPassword);
  }

  async findAll() {
    return this.prisma.user.findMany({
      include: { profile: true },
      orderBy: { dateJoined: 'desc' },
    });
  }

  private async generateUniqueSlug(username: string): Promise<string> {
    let slug = username.toLowerCase().replace(/\s+/g, '-');
    let count = 0;

    while (await this.prisma.profile.findUnique({ where: { slug } })) {
      count++;
      slug = `${username.toLowerCase().replace(/\s+/g, '-')}-${count}`;
    }

    return slug;
  }
}
