import { Button, GithubIcon, InView, Row, SlideUp, Stagger, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/links";
import { TrustAntiPatterns, TrustCard } from "@/components/sections";

import { NoCatchCopy } from "./NoCatchCopy";

/** "No catch" section — free + open source pitch + trust cards. */
export function NoCatchSection() {
  return (
    <InView>
      <div className="grid items-center gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
        <SlideUp>
          <NoCatchCopy />
        </SlideUp>
        <TrustCard
          footer={
            <Row justify="between">
              <Text size="xs" mono color="muted" className="uppercase tracking-wider">
                MIT Licensed &middot; Open Source
              </Text>
              <Button
                variant="secondary"
                href={GITHUB_URL}
                target="_blank"
                rel="noopener noreferrer"
              >
                <GithubIcon className="size-3.5" />
                GitHub
              </Button>
            </Row>
          }
        >
          <Stagger>
            <TrustAntiPatterns baseDelay={250} />
          </Stagger>
        </TrustCard>
      </div>
    </InView>
  );
}
