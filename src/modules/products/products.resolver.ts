import { Resolver, Query, Args, Int } from '@nestjs/graphql';
import { ProductsService } from './products.service';
import { Public } from '../../common/decorators/public.decorator';

@Resolver('Product')
export class ProductsResolver {
  constructor(private readonly productsService: ProductsService) {}

  @Public()
  @Query(() => String)
  async products(
    @Args('skip', { type: () => Int, nullable: true }) skip?: number,
    @Args('take', { type: () => Int, nullable: true }) take?: number,
  ) {
    const products = await this.productsService.findAll(skip, take);
    return JSON.stringify(products);
  }

  @Public()
  @Query(() => String)
  async product(@Args('id') id: string) {
    const product = await this.productsService.findOne(id);
    return JSON.stringify(product);
  }
}
