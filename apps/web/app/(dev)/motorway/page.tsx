"use client";

import { Heading, Stack, Tabs, TabsList, TabsTrigger, TabsContent, Text } from "@bnto/ui";

import { AnimationShowcase } from "./AnimationShowcase";
import { BorderShowcase } from "./BorderShowcase";
import { EmptyStateShowcase } from "./EmptyStateShowcase";
import { ButtonShowcase } from "./ButtonShowcase";
import { CheckboxShowcase } from "./CheckboxShowcase";
import { CopyButtonShowcase } from "./CopyButtonShowcase";
import { CardShowcase } from "./CardShowcase";
import { ColorSwatches } from "./ColorSwatches";
import { DialogShowcase } from "./DialogShowcase";
import { GridShowcase } from "./GridShowcase";
import { FileListShowcase } from "./FileListShowcase";
import { FormShowcase } from "./FormShowcase";
import { InputShowcase } from "./InputShowcase";
import { KeyValueEditorShowcase } from "./KeyValueEditorShowcase";
import { MenuShowcase } from "./MenuShowcase";
import { ListShowcase } from "./ListShowcase";
import { ProgressShowcase } from "./ProgressShowcase";
import { RadioGroupShowcase } from "./RadioGroupShowcase";
import { RecipeCardShowcase } from "./RecipeCardShowcase";
import { SelectShowcase } from "./SelectShowcase";
import { ShowcaseSection } from "./ShowcaseSection";
import { SliderShowcase } from "./SliderShowcase";
import { SwitchShowcase } from "./SwitchShowcase";
import { ComboboxShowcase } from "./ComboboxShowcase";
import { TextareaShowcase } from "./TextareaShowcase";
import { LoadingCardShowcase } from "./LoadingCardShowcase";
import { SchemaFormPlayground } from "./SchemaFormPlayground";
import { SpringableShowcase } from "./SpringableShowcase";
import { TypographyShowcase } from "./TypographyShowcase";

export default function MotorwayPage() {
  return (
    <div className="mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-8">
      {/* Header */}
      <div>
        <Heading level={1} size="md">
          Motorway
        </Heading>
        <Text color="muted" className="mt-1 max-w-xl">
          The bnto design system. Warm, organized, satisfying, like a well-packed bento box on a
          Mini Motorways map. 3D surfaces, springy motion, three themed palettes.
        </Text>
      </div>

      <Tabs defaultValue="surfaces" className="[&>[role=tabpanel]]:mt-8">
        <TabsList>
          <TabsTrigger value="surfaces">Surfaces</TabsTrigger>
          <TabsTrigger value="typography">Typography</TabsTrigger>
          <TabsTrigger value="controls">Controls</TabsTrigger>
          <TabsTrigger value="layout">Layout</TabsTrigger>
          <TabsTrigger value="motion">Motion</TabsTrigger>
          <TabsTrigger value="overlays">Overlays</TabsTrigger>
          <TabsTrigger value="forms">Forms</TabsTrigger>
          <TabsTrigger value="grids">Grids</TabsTrigger>
          <TabsTrigger value="progress">Progress</TabsTrigger>
          <TabsTrigger value="schemas">Schemas</TabsTrigger>
        </TabsList>

        {/* ── Surfaces ────────────────────────────────────────── */}
        <TabsContent value="surfaces">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="colors"
              title="Colors"
              description="Semantic color tokens across all variants. Each swatch renders with its matching surface tint."
            >
              <ColorSwatches />
            </ShowcaseSection>

            <ShowcaseSection
              id="elevation"
              title="Surface & Elevation"
              description="Cards cast directional shadows that follow the light source. Four elevation tiers from flush to floating."
            >
              <CardShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="border"
              title="Border"
              description="Three border modes for surfaces. Solid is the default, dashed for placeholder slots, none for borderless rows like list items."
            >
              <BorderShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="springable"
              title="Springable Surfaces"
              description="Configure spring mode and elevation independently. The spring foundation shared by all animated surfaces — pressable buttons and loading cards alike."
            >
              <SpringableShowcase />
            </ShowcaseSection>
          </Stack>
        </TabsContent>

        {/* ── Typography ──────────────────────────────────────── */}
        <TabsContent value="typography">
          <ShowcaseSection
            id="typography"
            title="Typography"
            description="Geist for display headings, Inter for body, Geist Mono for technical output."
          >
            <TypographyShowcase />
          </ShowcaseSection>
        </TabsContent>

        {/* ── Controls ──────────────────────────────────────── */}
        <TabsContent value="controls">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="buttons"
              title="Buttons"
              description="Three-span pushable DOM for blur-free spring animations. All variants, sizes, toggle mode, and dormant reveal."
            >
              <ButtonShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="copy-button"
              title="Copy Button"
              description="Ghost icon button that copies text to the clipboard. Swaps to a check icon on success. Use anywhere — log output, share links, code blocks."
            >
              <CopyButtonShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="selects"
              title="Selects"
              description="Dropdown selects with 3D surface popover. Grouped items, placeholder state, small trigger variant."
            >
              <SelectShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="combobox"
              title="Combobox"
              description="Multi-select combobox with search, badges, and remove. Built on cmdk + Popover. Used for file extensions, output formats, and any array-of-string params."
            >
              <ComboboxShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="inputs"
              title="Inputs"
              description="Text fields, password inputs, toggles, checkboxes, and radio groups."
            >
              <InputShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="textarea"
              title="Textarea"
              description="Multiline text input with size variants. Used for expressions, patterns, and long-form text in node config panels."
            >
              <TextareaShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="switch"
              title="Switch"
              description="Toggle switches for boolean params. On/off state with disabled variant."
            >
              <SwitchShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="checkbox"
              title="Checkbox"
              description="Checkboxes for boolean selections. Checked, unchecked, and disabled states."
            >
              <CheckboxShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="radio-group"
              title="Radio Group"
              description="Mutually exclusive option groups. Used for output format, size presets, and single-select enums."
            >
              <RadioGroupShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="sliders"
              title="Sliders"
              description="Linear sliders with optional presets and radial dial gauges. Used for quality, compression, and percentage params."
            >
              <SliderShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="key-value-editor"
              title="Key-Value Editor"
              description="Add/remove key→value pairs for record params. Used for column renames, header mappings, and transform definitions."
            >
              <KeyValueEditorShowcase />
            </ShowcaseSection>
          </Stack>
        </TabsContent>

        {/* ── Layout ──────────────────────────────────────────── */}
        <TabsContent value="layout">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="menus"
              title="Menus"
              description="Popover-based dropdowns. The trigger sinks into its pressed state when open."
            >
              <MenuShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="lists"
              title="Lists"
              description="Flat rows with Surface foundation. Notifications, grouped items with subheaders, selectable rows with border state, and action menus."
            >
              <ListShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="loading-card"
              title="Loading Cards"
              description="Pass loading to Card for the common case. Springs up with bounciest when content arrives. Skeleton placeholders swap to real content in place."
            >
              <LoadingCardShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="recipe-card"
              title="Recipe Card"
              description="Saved recipe cards for the dashboard grid. Shows name, node count, last run status, and relative timestamp. Includes skeleton loading state."
            >
              <RecipeCardShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="empty-state"
              title="Empty State"
              description="Composable empty state for lists, dashboards, and search results. Bare icon, title, description, and optional action. Three size variants."
            >
              <EmptyStateShowcase />
            </ShowcaseSection>
          </Stack>
        </TabsContent>

        {/* ── Motion ────────────────────────────────────────── */}
        <TabsContent value="motion">
          <ShowcaseSection
            id="motion"
            title="Motion"
            description="Entrances are springy. Elements pop onto the page like buildings materializing on the map. Transitions are smooth and restrained."
          >
            <AnimationShowcase />
          </ShowcaseSection>
        </TabsContent>

        {/* ── Overlays ──────────────────────────────────────────── */}
        <TabsContent value="overlays">
          <ShowcaseSection
            id="overlays"
            title="Dialogs & Overlays"
            description="Modal dialogs, confirmation prompts, form dialogs, and the AuthGate conversion patterns (Action + Section). Focus-trapped, keyboard accessible, backdrop blur."
          >
            <DialogShowcase />
          </ShowcaseSection>
        </TabsContent>

        {/* ── Forms ───────────────────────────────────────────── */}
        <TabsContent value="forms">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="forms"
              title="Form Composition"
              description="Controls composed into a realistic form inside an elevated card."
            >
              <FormShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="file-upload"
              title="File Upload"
              description="Drop zone with surface elevation, replaced by a staggered file list on selection. Clear to return to the drop zone."
            >
              <FileListShowcase />
            </ShowcaseSection>
          </Stack>
        </TabsContent>

        {/* ── Grids ───────────────────────────────────────────── */}
        <TabsContent value="grids">
          <ShowcaseSection
            id="grid"
            title="Grid"
            description="Explicit grid placement via composition. Items define their own position — the Grid provides the context."
          >
            <GridShowcase />
          </ShowcaseSection>
        </TabsContent>

        {/* ── Progress ──────────────────────────────────────────── */}
        <TabsContent value="progress">
          <ShowcaseSection
            id="progress-indicators"
            title="Progress Indicators"
            description="LinearProgress primitive at different values, with icons, labels, and helper text."
          >
            <ProgressShowcase />
          </ShowcaseSection>
        </TabsContent>

        {/* ── Schemas ─────────────────────────────────────────── */}
        <TabsContent value="schemas">
          <ShowcaseSection
            id="schema-form"
            title="Schema Form Inspector"
            description="Browse every engine-backed node type. See the live form, engine schema metadata, UI field configs, and inferred controls side by side."
          >
            <SchemaFormPlayground />
          </ShowcaseSection>
        </TabsContent>
      </Tabs>
    </div>
  );
}
