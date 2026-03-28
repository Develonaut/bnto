import Link from "next/link";
import { Row } from "@bnto/ui";
import { t } from "@bnto/i18n";

export function FooterBottom() {
  return (
    <div className="flex flex-col items-center justify-between gap-3 border-t border-border py-6 text-xs text-muted-foreground sm:flex-row">
      <p>{t("site.licenseLine")}</p>
      <Row className="gap-4">
        <Link href="/motorway" className="transition-colors hover:text-foreground">
          Motorway
        </Link>
        <Link href="/privacy" className="transition-colors hover:text-foreground">
          Privacy Policy
        </Link>
      </Row>
    </div>
  );
}
