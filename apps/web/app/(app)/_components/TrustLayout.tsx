import { GithubIcon } from "@bnto/ui";
import { ScaleIn, Button, Card, Center, Row, Stack, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/copy";

import { TrustAntiPatterns } from "./TrustAntiPatterns";

export function TrustLayout() {
  return (
    <ScaleIn from={0.9} easing="spring-bouncy">
      <Center className="w-full">
        <Card className="w-full max-w-sm p-6">
          <Stack className="gap-5">
            <TrustAntiPatterns />
            <div className="border-t border-border pt-4">
              <Row justify="between">
                <Text size="xs" mono color="muted" className="uppercase tracking-wider">
                  MIT Licensed &middot; Open Source
                </Text>
                <Button
                  variant="outline"
                  href={GITHUB_URL}
                  target="_blank"
                  rel="noopener noreferrer"
                  elevation="sm"
                >
                  <GithubIcon className="size-3.5" />
                  GitHub
                </Button>
              </Row>
            </div>
          </Stack>
        </Card>
      </Center>
    </ScaleIn>
  );
}
