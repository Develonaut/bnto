import {
  RecipeCardRoot,
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardFooter,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardStatus,
  RecipeCardTitle,
  RecipeCardDescription,
  RecipeCardTags,
  RecipeCardMeta,
  RecipeCardSkeleton,
} from "./RecipeCardRoot";

export const RecipeCard = Object.assign(RecipeCardRoot, {
  Root: RecipeCardRoot,
  Header: RecipeCardHeader,
  Content: RecipeCardContent,
  Footer: RecipeCardFooter,
  Icon: RecipeCardIcon,
  Category: RecipeCardCategory,
  Status: RecipeCardStatus,
  Title: RecipeCardTitle,
  Description: RecipeCardDescription,
  Tags: RecipeCardTags,
  Meta: RecipeCardMeta,
  Skeleton: RecipeCardSkeleton,
});
