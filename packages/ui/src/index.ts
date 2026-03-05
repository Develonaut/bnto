/* ── @bnto/ui — Motorway Design System ──────────────────────── */

/* Utils */
export { cn } from "./utils/cn";
export { createCn } from "./utils/createCn";
export { resolveResponsive } from "./utils/responsive";
export { toDropzoneAccept } from "./utils/toDropzoneAccept";
export { formatFileSize } from "./utils/formatFileSize";
export type { Breakpoints, ResponsiveProp } from "./utils/responsive";
export {
  resolveGap,
  alignMap,
  justifyMap,
  paddingMap,
  paddingXMap,
  paddingYMap,
} from "./utils/layoutTypes";
export type { GapSize, ResponsiveGap, Align, Justify, LayoutElement } from "./utils/layoutTypes";

/* Icons */
export * from "./icons";

/* Hooks */
export { usePrevious } from "./hooks/usePrevious";

/* Layout */
export { AppShell, AppShellHeader, AppShellMain, AppShellContent } from "./layout/AppShell";
export { BentoGrid, BentoGridPinned } from "./layout/BentoGrid";
export { assignCellLayouts } from "./layout/assignCellLayouts";
export { useBentoItem } from "./layout/useBentoItem";
export type { CellLayout } from "./layout/bentoGridContext";
export { Center } from "./layout/Center";
export { Container } from "./layout/Container";
export { Grid, GridItem } from "./layout/Grid";
export { Inset } from "./layout/Inset";
export { Row } from "./layout/Row";
export { Stack } from "./layout/Stack";

/* Typography */
export { Badge } from "./typography/Badge";
export { Heading } from "./typography/Heading";
export { IconBadge } from "./typography/IconBadge";
export { Label } from "./typography/Label";
export { Text } from "./typography/Text";

/* Feedback */
export { ComparisonBar } from "./feedback/ComparisonBar";
export {
  EmptyState,
  EmptyStateIcon,
  EmptyStateTitle,
  EmptyStateDescription,
  EmptyStateAction,
  EmptyStateSkeleton,
} from "./feedback/EmptyState";
export { LinearProgress } from "./feedback/LinearProgress";
export { Skeleton } from "./feedback/Skeleton";

/* Surface */
export {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "./surface/Card";
export { Divider } from "./surface/Divider";
export {
  Panel,
  PanelHeader,
  PanelTrigger,
  PanelDivider,
  PanelContent,
  PanelFooter,
} from "./surface/Panel";
export { Pressable, SPRING_STYLES } from "./surface/Pressable";
export { Surface } from "./surface/Surface";
export type { SurfaceVariant, SurfaceElevation, SurfaceRounded } from "./surface/Surface";
export { Toolbar, ToolbarGroup, ToolbarDivider } from "./surface/Toolbar";

/* Interaction */
export {
  Accordion,
  AccordionItem,
  AccordionTrigger,
  AccordionContent,
} from "./interaction/Accordion";
export { Button, buttonCn } from "./interaction/Button";
export type { SpringMode } from "./interaction/Button";
export { Checkbox } from "./interaction/Checkbox";
export {
  FileUpload,
  FileUploadDropzone,
  FileUploadList,
  FileUploadItem,
  FileUploadItemMetadata,
  FileUploadItemActions,
  FileUploadItemDelete,
  FileUploadClear,
} from "./interaction/FileUpload";
export type { FileUploadProps } from "./interaction/FileUpload";
export { Input } from "./interaction/Input";
export {
  Menu,
  MenuTrigger,
  MenuContent,
  MenuClose,
  MenuItem,
  MenuLabel,
  MenuSeparator,
} from "./interaction/Menu";
export { PasswordInput } from "./interaction/PasswordInput";
export { RadialSlider } from "./interaction/RadialSlider";
export { RadioGroup, RadioGroupItem } from "./interaction/RadioGroup";
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
} from "./interaction/Select";
export { Slider } from "./interaction/Slider";
export { Switch } from "./interaction/Switch";
export { Tabs, TabsList, TabsTrigger, TabsContent } from "./interaction/Tabs";
export { Textarea } from "./interaction/Textarea";

/* Overlay */
export {
  Dialog,
  DialogTrigger,
  DialogClose,
  DialogPortal,
  DialogOverlay,
  DialogContent,
  DialogHeader,
  DialogBody,
  DialogFooter,
  DialogTitle,
  DialogDescription,
} from "./overlay/Dialog";
export {
  Popover,
  PopoverTrigger,
  PopoverAnchor,
  PopoverClose,
  PopoverPortal,
  PopoverContent,
  PopoverContentUnstyled,
} from "./overlay/Popover";
export {
  Sheet,
  SheetTrigger,
  SheetClose,
  SheetOverlay,
  SheetContent,
  SheetHeader,
  SheetFooter,
  SheetTitle,
  SheetDescription,
} from "./overlay/Sheet";

/* Animation */
export {
  Stagger,
  ScaleIn,
  FadeIn,
  SlideUp,
  SlideDown,
  PulseSoft,
  Breathe,
  BouncyStagger,
  InView,
} from "./animation/Animate";
export { AnimatedCounter } from "./animation/AnimatedCounter";
export { AnimatedThemeToggle } from "./animation/AnimatedThemeToggle";
