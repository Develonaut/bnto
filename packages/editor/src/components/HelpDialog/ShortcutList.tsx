import { Kbd, KbdGroup, Row, Text } from "@bnto/ui";
import { SHORTCUTS } from "../../utils/shortcuts";
import { getShortcutKeys } from "../../utils/getShortcutKeys";

/** Renders the list of keyboard shortcuts, platform-aware. */
function ShortcutList({ isMac }: { isMac: boolean }) {
  return (
    <>
      {SHORTCUTS.map((shortcut) => {
        const keys = isMac ? shortcut.mac : getShortcutKeys(shortcut);
        return (
          <Row key={shortcut.id} className="items-center justify-between px-4 py-2">
            <Text size="sm">{shortcut.label}</Text>
            <KbdGroup>
              {keys.map((key) => (
                <Kbd key={key}>{key}</Kbd>
              ))}
            </KbdGroup>
          </Row>
        );
      })}
    </>
  );
}

export { ShortcutList };
