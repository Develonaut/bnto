import { Accordion, AccordionContent, AccordionItem, AccordionTrigger, SlideUp } from "@bnto/ui";
import { SectionHeader, SectionShell } from "@/components/sections";
import { getFaq } from "@/lib/recipePageContent";
import { FAQ_MASCOT } from "../_utils/sectionMascots";

interface RecipeFaqSectionProps {
  slug: string;
  category: string;
}

export function RecipeFaqSection({ slug, category }: RecipeFaqSectionProps) {
  const items = getFaq(slug, category);
  if (items.length === 0) return null;

  return (
    <SectionShell size="md">
      <SectionHeader title="Common questions" mascot={FAQ_MASCOT} mascotHeight={200} />
      <SlideUp index={1}>
        <Accordion type="single" collapsible className="w-full">
          {items.map((item, i) => (
            <AccordionItem key={i} value={`recipe-faq-${i}`}>
              <AccordionTrigger className="text-left hover:no-underline">
                {item.question}
              </AccordionTrigger>
              <AccordionContent className="text-muted-foreground">{item.answer}</AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>
      </SlideUp>
    </SectionShell>
  );
}
