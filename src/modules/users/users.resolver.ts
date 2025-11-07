import { Resolver, Query, Args } from '@nestjs/graphql';
import { UseGuards } from '@nestjs/common';
import { UsersService } from './users.service';
import { User } from './entities/user.entity';
import { GqlAuthGuard } from '../../common/guards/gql-auth.guard';
import { CurrentUser } from '../../common/decorators/current-user.decorator';

@Resolver(() => User)
export class UsersResolver {
  constructor(private readonly usersService: UsersService) {}

  @Query(() => User, { name: 'me' })
  @UseGuards(GqlAuthGuard)
  async getCurrentUser(@CurrentUser() user: any) {
    return this.usersService.findById(user.id);
  }

  @Query(() => [User], { name: 'users' })
  @UseGuards(GqlAuthGuard)
  async findAll() {
    return this.usersService.findAll();
  }

  @Query(() => User, { name: 'user' })
  @UseGuards(GqlAuthGuard)
  async findOne(@Args('id') id: string) {
    return this.usersService.findById(id);
  }
}
