import { useState } from 'react';
import { ChevronDown, ChevronUp } from 'lucide-react';
import { gql, useQuery } from '@apollo/client';

const GET_CATEGORIES = gql`
  query GetCategories {
    allSuperCategories {
      id
      name
      slug
      categoryImage
    }
  }
`;

interface CategorySidebarProps {
  onCategoryClick?: (slug: string) => void;
}

export function CategorySidebar({ onCategoryClick }: CategorySidebarProps) {
  const [showMore, setShowMore] = useState(false);
  const { data, loading } = useQuery(GET_CATEGORIES);

  const categories = data?.allSuperCategories || [];
  const displayCategories = showMore ? categories : categories.slice(0, 10);

  const handleCategoryClick = (slug: string) => {
    if (onCategoryClick) {
      onCategoryClick(slug);
    }
  };

  if (loading) {
    return (
      <div className="categories-dropdown-wrap">
        <div className="animate-pulse">
          {[...Array(5)].map((_, i) => (
            <div key={i} className="h-12 bg-gray-200 mb-2 rounded"></div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="categories-dropdown-wrap style-2 font-heading mt-30 bg-white rounded-lg shadow-sm">
      <div className="categori-dropdown-inner">
        <ul className="space-y-1">
          {displayCategories.map((category: any) => (
            <li key={category.id} className="py-3 px-4 hover:bg-brand-light transition-colors cursor-pointer border-b border-gray-100 last:border-b-0">
              <a
                href={`/category/${category.slug}`}
                onClick={(e) => {
                  e.preventDefault();
                  handleCategoryClick(category.slug);
                }}
                className="flex items-center space-x-3"
              >
                {category.categoryImage ? (
                  <img
                    src={category.categoryImage}
                    alt={category.name}
                    className="w-8 h-8 object-cover rounded"
                    onError={(e) => {
                      (e.target as HTMLImageElement).src = '/assets/imgs/theme/icons/category-1.svg';
                    }}
                  />
                ) : (
                  <div className="w-8 h-8 bg-brand-light rounded flex items-center justify-center">
                    <span className="text-brand font-semibold text-xs">
                      {category.name.charAt(0)}
                    </span>
                  </div>
                )}
                <span className="text-gray-700 hover:text-brand font-medium">
                  {category.name}
                </span>
              </a>
            </li>
          ))}
        </ul>
      </div>

      {categories.length > 10 && (
        <div
          className="more_categories py-3 px-4 cursor-pointer border-t bg-gray-50 hover:bg-gray-100 transition-colors flex items-center justify-center"
          onClick={() => setShowMore(!showMore)}
        >
          {showMore ? (
            <>
              <ChevronUp className="w-4 h-4 mr-2" />
              <span className="heading-sm-1 font-semibold text-brand">Show less...</span>
            </>
          ) : (
            <>
              <ChevronDown className="w-4 h-4 mr-2" />
              <span className="heading-sm-1 font-semibold text-brand">Show more...</span>
            </>
          )}
        </div>
      )}
    </div>
  );
}
