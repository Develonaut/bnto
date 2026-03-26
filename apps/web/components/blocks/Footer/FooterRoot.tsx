import { Container } from "@bnto/ui";
import { FooterBrand } from "./FooterBrand";
import { FooterLinks } from "./FooterLinks";
import { FooterBottom } from "./FooterBottom";

export function FooterRoot() {
  return (
    <footer className="border-t border-border" data-testid="footer">
      <Container>
        <div className="flex flex-col gap-10 py-12 lg:flex-row lg:gap-20 lg:py-16">
          <FooterBrand />
          <FooterLinks />
        </div>
        <FooterBottom />
      </Container>
    </footer>
  );
}
