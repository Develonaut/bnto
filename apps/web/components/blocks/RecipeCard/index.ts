/* Generic sub-components — sourced from @bnto/ui */
export {
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardFooter,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardTitle,
  RecipeCardDescription,
  RecipeCardTags,
} from "@bnto/ui";

/* App-layer root — adds Next.js Link support */
export { RecipeCardRoot as RecipeCard } from "./RecipeCardRoot";

/* Domain sub-components — status, meta (not in @bnto/ui) */
export { RecipeCardStatus, RecipeCardMeta } from "./RecipeCardRoot";
