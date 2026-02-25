import { Button } from "@/components/ui/button";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { GithubIcon, HeartIcon, SettingsIcon, StarIcon, SwatchBookIcon, ZapIcon } from "@/components/ui/icons";

export function PressableShowcase() {
  return (
    <Stack className="gap-8">
      {/* Alignment test — mixed states in a row */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Mixed states in a row — bottom edges should align</Text>
        <Row className="items-end gap-3">
          <Button variant="outline">Default</Button>
          <Button variant="outline" pseudo="hover">Hover</Button>
          <Button variant="outline" pseudo="active">Active</Button>
          <Button variant="outline">Default</Button>
        </Row>
      </div>

      {/* Icon buttons alignment */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Icon buttons — mixed states</Text>
        <Row className="items-end gap-3">
          <Button variant="outline" size="icon"><StarIcon /></Button>
          <Button variant="outline" size="icon" pseudo="hover"><HeartIcon /></Button>
          <Button variant="outline" size="icon" pseudo="active"><ZapIcon /></Button>
          <Button variant="outline" size="icon"><SettingsIcon /></Button>
          <Button variant="outline" size="icon" pseudo="hover"><SwatchBookIcon /></Button>
          <Button variant="outline" size="icon"><GithubIcon /></Button>
        </Row>
      </div>

      {/* Navbar simulation */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Navbar simulation — one active, rest default</Text>
        <Row className="items-center gap-2 rounded-full border bg-card px-6 py-3">
          <Button variant="outline" depth="sm">Sign in</Button>
          <Button variant="outline" size="icon" depth="sm" pseudo="hover"><SwatchBookIcon /></Button>
          <Button variant="outline" size="icon" depth="sm"><GithubIcon /></Button>
          <Button variant="outline" size="icon" depth="sm"><SettingsIcon /></Button>
        </Row>
      </div>

      {/* Different depths */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Depth comparison — sm, md, lg with hover forced</Text>
        <Row className="items-end gap-4">
          <Stack className="items-center gap-1">
            <Button variant="outline" depth="sm">sm</Button>
            <Button variant="outline" depth="sm" pseudo="hover">sm hover</Button>
          </Stack>
          <Stack className="items-center gap-1">
            <Button variant="outline">md (default)</Button>
            <Button variant="outline" pseudo="hover">md hover</Button>
          </Stack>
          <Stack className="items-center gap-1">
            <Button variant="outline" depth="lg">lg</Button>
            <Button variant="outline" depth="lg" pseudo="hover">lg hover</Button>
          </Stack>
        </Row>
      </div>

      {/* All variants with hover */}
      <div>
        <Text size="sm" color="muted" className="mb-3">All variants — default vs hover</Text>
        <Row className="flex-wrap items-end gap-3">
          <Button variant="primary">Primary</Button>
          <Button variant="primary" pseudo="hover">Primary</Button>
          <Button variant="secondary">Secondary</Button>
          <Button variant="secondary" pseudo="hover">Secondary</Button>
          <Button variant="outline">Outline</Button>
          <Button variant="outline" pseudo="hover">Outline</Button>
          <Button variant="destructive">Destructive</Button>
          <Button variant="destructive" pseudo="hover">Destructive</Button>
        </Row>
      </div>
    </Stack>
  );
}
