import { Heading, Text } from "@bnto/ui";
import { SectionHeader, SectionShell, TrustCard } from "@/components/sections";
import { TRUST_MASCOT } from "../_utils/sectionMascots";

const VALUE_PROPS = [
  {
    title: "Browser-based privacy",
    body: "Files never leave your device. The Rust engine compiles to WebAssembly and runs entirely in your browser tab.",
  },
  {
    title: "No signup required",
    body: "Open the page, drop your files, download results. No account, no email, no credit card.",
  },
  {
    title: "Free forever",
    body: "No daily caps, no watermarks, no quality limits. The engine runs on your hardware, so it costs nothing to operate.",
  },
  {
    title: "Open source",
    body: "The entire engine, CLI, and web app are MIT licensed. Inspect the code, verify the operations, or contribute.",
  },
] as const;

export function WhyBntoSection() {
  return (
    <SectionShell muted>
      <SectionHeader label="Why bnto" title="No catches. No compromises." />
      <TrustCard mascot={TRUST_MASCOT} mascotSize={200}>
        <div className="grid gap-4 sm:grid-cols-2">
          {VALUE_PROPS.map((prop, i) => (
            <div key={i}>
              <Heading level={3} size="sm" className="mb-1">
                {prop.title}
              </Heading>
              <Text color="muted" leading="snug" size="sm">
                {prop.body}
              </Text>
            </div>
          ))}
        </div>
      </TrustCard>
    </SectionShell>
  );
}
