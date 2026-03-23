/**
 * Page-level content header — title on the left, optional actions on the right.
 *
 * Distinct from the app-level ContentHeader (sticky top bar with nav actions).
 * This sits inside the page content area below the app header.
 */

import type { ReactNode } from "react";

import { Heading, Row } from "@bnto/ui";

interface PageContentHeaderProps {
  title: string;
  /** Optional right-side actions (e.g. filter menu, buttons). */
  children?: ReactNode;
  "data-testid"?: string;
}

export function PageContentHeader({ title, children, ...props }: PageContentHeaderProps) {
  return (
    <Row className="items-center justify-between">
      <Heading level={1} size="lg" data-testid={props["data-testid"]}>
        {title}
      </Heading>
      {children}
    </Row>
  );
}
