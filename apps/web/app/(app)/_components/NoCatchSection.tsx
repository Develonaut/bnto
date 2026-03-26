import { InView } from "@bnto/ui";
import { NoCatchCopy } from "./NoCatchCopy";
import { TrustLayout } from "./TrustLayout";

/** "No catch" section — free + open source pitch + trust cards. */
export function NoCatchSection() {
  return (
    <InView>
      <div className="grid items-center gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
        <NoCatchCopy />
        <TrustLayout />
      </div>
    </InView>
  );
}
