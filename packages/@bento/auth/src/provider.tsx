import { ConvexAuthNextjsServerProvider } from "@convex-dev/auth/nextjs/server";

export function BentoAuthProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ConvexAuthNextjsServerProvider>
      {children}
    </ConvexAuthNextjsServerProvider>
  );
}
