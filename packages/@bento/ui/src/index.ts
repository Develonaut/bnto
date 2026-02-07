/**
 * @bento/ui — Design system and shared components.
 *
 * Thin wrappers around shadcn primitives.
 * Used by both web (Next.js) and desktop (Wails/Vite) apps.
 */

export { cn } from "./lib/utils";

export { Button, buttonVariants } from "./components/shared/button";
export type { ButtonProps } from "./components/shared/button";

export { Input } from "./components/shared/input";
export type { InputProps } from "./components/shared/input";

export { Label } from "./components/shared/label";

export {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "./components/shared/card";

export { ThemeToggle } from "./components/shared/theme-toggle";
