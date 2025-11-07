import { ObjectType, Field, ID, registerEnumType } from '@nestjs/graphql';
import { UserRole } from '@prisma/client';

registerEnumType(UserRole, {
  name: 'UserRole',
});

@ObjectType()
export class User {
  @Field(() => ID)
  id: string;

  @Field()
  email: string;

  @Field()
  username: string;

  @Field({ nullable: true })
  firstName?: string;

  @Field({ nullable: true })
  lastName?: string;

  @Field(() => UserRole)
  role: UserRole;

  @Field()
  isActive: boolean;

  @Field()
  isStaff: boolean;

  @Field()
  isSuperuser: boolean;

  @Field()
  dateJoined: Date;

  @Field({ nullable: true })
  lastLogin?: Date;
}
