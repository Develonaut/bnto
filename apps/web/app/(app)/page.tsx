import {
  AppShellContent,
  Divider,
  Heading,
  InView,
  Stack,
  Text,
} from "@bnto/ui";
import { HeroSidebar } from "@/components/blocks/HeroSidebar";
import { RecipeGrid } from "@/components/blocks/RecipeGrid";

import { BragLayout } from "./_components/BragLayout";
import { TrustLayout } from "./_components/TrustLayout";

/* ── Home page ───────────────────────────────────────────────── */

export default function Home() {
  return (
    <AppShellContent>
      {/* Hero — sidebar + recipe grid */}
      <div className="grid items-start gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
        <HeroSidebar showCta={false} />
        <RecipeGrid />
      </div>

      <Divider label="Free. No signup." />

      {/* How it works — copy + brag cards */}
      <InView>
        <div className="grid items-center gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
          <Stack gap="md">
            <Text size="sm" mono color="muted" className="uppercase tracking-wider">
              How it works
            </Text>
            <Heading level={2} size="xl" className="whitespace-pre-line">
              {"Your browser does the work.\nNot a server."}
            </Heading>
            <Text color="muted" leading="snug">
              Other tools upload your files to a server, process them remotely, and send the results
              back. That takes time, and it means your files leave your device. bnto runs entirely
              in your browser. Processing happens on your machine in milliseconds. Nothing is
              uploaded. Nothing leaves.
            </Text>
          </Stack>
          <BragLayout />
        </div>
      </InView>

      <Divider label="No catch." />

      {/* No catch — copy + trust card */}
      <InView>
        <div className="grid items-center gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
          <Stack gap="md">
            <Text size="sm" mono color="muted" className="uppercase tracking-wider">
              No catch
            </Text>
            <Heading level={2} size="xl" className="whitespace-pre-line">
              {"Free tools that stay free.\nYour browser does the work."}
            </Heading>
            <Text color="muted" leading="snug">
              No signup. No watermarks. No daily caps. No &apos;20 free compressions per
              month.&apos; Your browser does all the processing, so it costs us nothing to run.
              We&apos;ll never put a meter on it.
            </Text>
          </Stack>
          <TrustLayout />
        </div>
      </InView>
    </AppShellContent>
  );
}
