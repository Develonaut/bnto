import { ScaleIn, Stack, Stagger } from "@bnto/ui";

import { BragSpeedCard } from "./BragSpeedCard";
import { BragCapabilityCard } from "./BragCapabilityCard";

export function BragLayout() {
  return (
    <Stagger asChild>
      <Stack className="gap-3">
        <ScaleIn index={0} from={0.9} easing="spring-bouncy">
          <BragSpeedCard />
        </ScaleIn>
        <ScaleIn index={1} from={0.9} easing="spring-bouncy">
          <BragCapabilityCard />
        </ScaleIn>
      </Stack>
    </Stagger>
  );
}
