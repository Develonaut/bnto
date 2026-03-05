/* ── @bnto/ui — Motorway Design System ──────────────────────── */

/* Utils */
export { cn } from "./utils/cn";
export { createCn } from "./utils/createCn";
export { resolveResponsive } from "./utils/responsive";
export type { Breakpoints, ResponsiveProp } from "./utils/responsive";
export {
  resolveGap,
  alignMap,
  justifyMap,
  paddingMap,
  paddingXMap,
  paddingYMap,
} from "./utils/layoutTypes";
export type {
  GapSize,
  ResponsiveGap,
  Align,
  Justify,
  LayoutElement,
} from "./utils/layoutTypes";

/* Icons */
export * from "./icons";

/* Hooks */
export { usePrevious } from "./hooks/usePrevious";

/* Layout */
export { AppShell } from "./layout/AppShell";
export { BentoGrid } from "./layout/BentoGrid";
export { assignCellLayouts } from "./layout/assignCellLayouts";
export { useBentoItem } from "./layout/useBentoItem";
export type { CellLayout } from "./layout/bentoGridContext";
export { Center } from "./layout/Center";
export { Container } from "./layout/Container";
export { Grid } from "./layout/Grid";
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
export { EmptyState } from "./feedback/EmptyState";
export { LinearProgress } from "./feedback/LinearProgress";
export { Skeleton } from "./feedback/Skeleton";

/* Surface */
export { Card } from "./surface/Card";
export { Divider } from "./surface/Divider";
export { Panel } from "./surface/Panel";
export { Pressable, SPRING_STYLES } from "./surface/Pressable";
export { Surface } from "./surface/Surface";
export { Toolbar } from "./surface/Toolbar";

/* Interaction */
export { Accordion } from "./interaction/Accordion";
export { Button, buttonCn } from "./interaction/Button";
export type { SpringMode } from "./interaction/Button";
export { Checkbox } from "./interaction/Checkbox";
export { FileUpload } from "./interaction/FileUpload";
export type { FileUploadProps } from "./interaction/FileUpload";
export { Input } from "./interaction/Input";
export { Menu } from "./interaction/Menu";
export { PasswordInput } from "./interaction/PasswordInput";
export { RadialSlider } from "./interaction/RadialSlider";
export { RadioGroup } from "./interaction/RadioGroup";
export { Select } from "./interaction/Select";
export { Slider } from "./interaction/Slider";
export { Switch } from "./interaction/Switch";
export { Tabs } from "./interaction/Tabs";
export { Textarea } from "./interaction/Textarea";

/* Overlay */
export { Dialog } from "./overlay/Dialog";
export { Popover } from "./overlay/Popover";
export { Sheet } from "./overlay/Sheet";

/* Animation */
export { Animate, BouncyStagger } from "./animation/Animate";
export { AnimatedCounter } from "./animation/AnimatedCounter";
export { AnimatedThemeToggle } from "./animation/AnimatedThemeToggle";
export { InView } from "./animation/InView";
