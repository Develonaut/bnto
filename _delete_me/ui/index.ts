/**
 * @bnto/ui — Design system and shared components.
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

export {
  Dialog,
  DialogTrigger,
  DialogPortal,
  DialogClose,
  DialogOverlay,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
  DialogDescription,
} from "./components/shared/dialog";

export {
  Select,
  SelectGroup,
  SelectValue,
  SelectTrigger,
  SelectContent,
  SelectLabel,
  SelectItem,
  SelectSeparator,
  SelectScrollUpButton,
  SelectScrollDownButton,
} from "./components/shared/select";

export {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "./components/shared/tabs";

export { Toaster } from "./components/shared/toaster";
