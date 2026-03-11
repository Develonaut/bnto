import type { ReactNode } from "react";

/**
 * NodeBody — center zone of the node card grid.
 *
 * Overlays the single grid cell with `place-self-center` so content
 * is perfectly centered both vertically and horizontally. Unaffected
 * by header/footer/edge overlays — they don't displace the body.
 */

function NodeBody({ children }: { children: ReactNode }) {
  return (
    <div className="col-start-1 row-start-1 place-self-center flex flex-col items-center justify-center gap-1">
      {children}
    </div>
  );
}

export { NodeBody };
