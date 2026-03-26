import { Heading, InView, Stack, Text } from "@bnto/ui";
import { BragLayout } from "./BragLayout";

/** "How it works" section — browser-first processing pitch + brag cards. */
export function HowItWorksSection() {
  return (
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
            back. That takes time, and it means your files leave your device. bnto runs entirely in
            your browser. Processing happens on your machine in milliseconds. Nothing is uploaded.
            Nothing leaves.
          </Text>
        </Stack>
        <BragLayout />
      </div>
    </InView>
  );
}
