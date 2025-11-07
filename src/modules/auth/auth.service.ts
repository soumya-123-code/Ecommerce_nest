import { Injectable, UnauthorizedException, ConflictException } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { UsersService } from '../users/users.service';
import { RegisterInput } from './dto/register.input';
import { LoginInput } from './dto/login.input';

@Injectable()
export class AuthService {
  constructor(
    private readonly usersService: UsersService,
    private readonly jwtService: JwtService,
  ) {}

  async register(registerInput: RegisterInput) {
    const existingUserByEmail = await this.usersService.findByEmail(registerInput.email);
    if (existingUserByEmail) {
      throw new ConflictException('Email already exists');
    }

    const existingUserByUsername = await this.usersService.findByUsername(registerInput.username);
    if (existingUserByUsername) {
      throw new ConflictException('Username already exists');
    }

    const user = await this.usersService.create({
      email: registerInput.email,
      username: registerInput.username,
      password: registerInput.password,
      firstName: registerInput.firstName,
      lastName: registerInput.lastName,
      role: registerInput.role,
    });

    const accessToken = this.generateToken(user.id, user.email, user.role);

    return {
      accessToken,
      user,
    };
  }

  async login(loginInput: LoginInput) {
    const user = await this.usersService.findByEmailOrUsername(loginInput.usernameOrEmail);

    if (!user) {
      throw new UnauthorizedException('Invalid credentials');
    }

    const isPasswordValid = await this.usersService.validatePassword(
      loginInput.password,
      user.password,
    );

    if (!isPasswordValid) {
      throw new UnauthorizedException('Invalid credentials');
    }

    if (!user.isActive) {
      throw new UnauthorizedException('Account is deactivated');
    }

    const accessToken = this.generateToken(user.id, user.email, user.role);

    return {
      accessToken,
      user,
    };
  }

  async validateUser(userId: string) {
    return this.usersService.findById(userId);
  }

  private generateToken(userId: string, email: string, role: string): string {
    const payload = { sub: userId, email, role };
    return this.jwtService.sign(payload);
  }
}
