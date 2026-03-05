"use client";

import { useState } from "react";
import Link from "next/link";

import { Menu, MenuTrigger, MenuContent, MenuItem, MenuLabel, MenuSeparator, Row } from "@bnto/ui";

const DEMO_LINKS = [
  {
    label: "Compress Images",
    description: "Shrink PNG, JPEG, and WebP without losing quality",
    url: "#",
  },
  {
    label: "Resize Images",
    description: "Scale images to exact dimensions or percentages",
    url: "#",
  },
  { label: "Convert Format", description: "Switch between PNG, JPEG, WebP, and GIF", url: "#" },
  { label: "Clean CSV", description: "Remove empty rows, trim whitespace, deduplicate", url: "#" },
];

export function MenuShowcase() {
  const [boundaryEl, setBoundaryEl] = useState<HTMLDivElement | null>(null);

  return (
    <div ref={setBoundaryEl} className="space-y-8">
      <Row className="flex-wrap gap-6">
        {/* Basic menu — rich items with description */}
        <Menu>
          <MenuTrigger variant="outline">Recipes</MenuTrigger>
          <MenuContent className="w-[28rem] p-3" boundary={boundaryEl}>
            <div className="grid grid-cols-2 gap-1">
              {DEMO_LINKS.map((link) => (
                <MenuItem key={link.label} asChild>
                  <Link href={link.url} className="flex-col items-start no-underline">
                    <span className="text-sm leading-normal font-medium">{link.label}</span>
                    <span className="text-muted-foreground text-xs leading-normal">
                      {link.description}
                    </span>
                  </Link>
                </MenuItem>
              ))}
            </div>
          </MenuContent>
        </Menu>

        {/* Primary variant trigger — action items */}
        <Menu>
          <MenuTrigger variant="primary">Actions</MenuTrigger>
          <MenuContent className="w-56 p-2" boundary={boundaryEl}>
            <MenuItem>New Recipe</MenuItem>
            <MenuItem>Import</MenuItem>
            <MenuItem>Export</MenuItem>
          </MenuContent>
        </Menu>

        {/* Secondary variant trigger — with labels and separator */}
        <Menu>
          <MenuTrigger variant="secondary">Options</MenuTrigger>
          <MenuContent className="w-48 p-2" elevation="sm" boundary={boundaryEl}>
            <MenuLabel>Actions</MenuLabel>
            <MenuItem>Edit</MenuItem>
            <MenuItem>Duplicate</MenuItem>
            <MenuSeparator />
            <MenuItem className="text-destructive">Delete</MenuItem>
          </MenuContent>
        </Menu>
      </Row>
    </div>
  );
}
