import { BentoAuthProvider } from "@bento/auth/provider";
import { ThemeProvider } from "./theme-provider";

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <BentoAuthProvider>
      <ThemeProvider>{children}</ThemeProvider>
    </BentoAuthProvider>
  );
}
