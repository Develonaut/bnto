"use client";

import { useState } from "react";
import Link from "next/link";

import { Button } from "@/components/ui/Button";
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
        {/* Basic menu */}
        <Menu>
          <Menu.Trigger variant="outline">Recipes</Menu.Trigger>
          <Menu.Content className="w-[28rem] p-3" boundary={boundaryEl}>
            <ul className="grid grid-cols-2 gap-1">
              {DEMO_LINKS.map((link) => (
                <li key={link.label}>
                  <Link
                    href={link.url}
                    className="flex flex-col gap-1 rounded-lg px-3 py-2.5 no-underline outline-hidden transition-colors select-none hover:bg-muted focus:bg-muted"
                  >
                    <span className="text-sm leading-normal font-medium">
                      {link.label}
                    </span>
                    <span className="text-muted-foreground text-xs leading-normal">
                      {link.description}
                    </span>
                  </Link>
                </li>
              ))}
            </ul>
          </Menu.Content>
        </Menu>

        {/* Primary variant trigger */}
        <Menu>
          <Menu.Trigger variant="primary">Actions</Menu.Trigger>
          <Menu.Content className="w-56 p-2" boundary={boundaryEl}>
            <ul className="space-y-1">
              <li>
                <Button variant="secondary" className="w-full justify-start" elevation={false}>
                  New Recipe
                </Button>
              </li>
              <li>
                <Button variant="secondary" className="w-full justify-start" elevation={false}>
                  Import
                </Button>
              </li>
              <li>
                <Button variant="secondary" className="w-full justify-start" elevation={false}>
                  Export
                </Button>
              </li>
            </ul>
          </Menu.Content>
        </Menu>

        {/* Secondary variant trigger */}
        <Menu>
          <Menu.Trigger variant="secondary">Options</Menu.Trigger>
          <Menu.Content className="w-48 p-2" elevation="sm" boundary={boundaryEl}>
            <ul className="space-y-1 text-sm">
              <li className="px-3 py-2 rounded-lg hover:bg-muted cursor-pointer">Edit</li>
              <li className="px-3 py-2 rounded-lg hover:bg-muted cursor-pointer">Duplicate</li>
              <li className="px-3 py-2 rounded-lg hover:bg-muted cursor-pointer text-destructive">Delete</li>
            </ul>
          </Menu.Content>
        </Menu>
      </Row>
    </div>
  );
}
