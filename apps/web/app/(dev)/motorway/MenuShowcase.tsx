"use client";

import { useState } from "react";
import Link from "next/link";

import { Menu } from "@/components/ui/Menu";
import { Row } from "@/components/ui/Row";

const DEMO_LINKS = [
  { label: "Compress Images", description: "Shrink PNG, JPEG, and WebP without losing quality", url: "#" },
  { label: "Resize Images", description: "Scale images to exact dimensions or percentages", url: "#" },
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
          <Menu.Trigger variant="outline">Recipes</Menu.Trigger>
          <Menu.Content className="w-[28rem] p-3" boundary={boundaryEl}>
            <div className="grid grid-cols-2 gap-1">
              {DEMO_LINKS.map((link) => (
                <Menu.Item key={link.label} asChild>
                  <Link href={link.url} className="flex-col items-start no-underline">
                    <span className="text-sm leading-normal font-medium">
                      {link.label}
                    </span>
                    <span className="text-muted-foreground text-xs leading-normal">
                      {link.description}
                    </span>
                  </Link>
                </Menu.Item>
              ))}
            </div>
          </Menu.Content>
        </Menu>

        {/* Primary variant trigger — action items */}
        <Menu>
          <Menu.Trigger variant="primary">Actions</Menu.Trigger>
          <Menu.Content className="w-56 p-2" boundary={boundaryEl}>
            <Menu.Item>New Recipe</Menu.Item>
            <Menu.Item>Import</Menu.Item>
            <Menu.Item>Export</Menu.Item>
          </Menu.Content>
        </Menu>

        {/* Secondary variant trigger — with labels and separator */}
        <Menu>
          <Menu.Trigger variant="secondary">Options</Menu.Trigger>
          <Menu.Content className="w-48 p-2" elevation="sm" boundary={boundaryEl}>
            <Menu.Label>Actions</Menu.Label>
            <Menu.Item>Edit</Menu.Item>
            <Menu.Item>Duplicate</Menu.Item>
            <Menu.Separator />
            <Menu.Item className="text-destructive">Delete</Menu.Item>
          </Menu.Content>
        </Menu>
      </Row>
    </div>
  );
}
