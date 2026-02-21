import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const { passphrase } = (await request.json()) as { passphrase?: string };

  if (!passphrase) {
    return NextResponse.json({ valid: false }, { status: 400 });
  }

  const secret = process.env.BNTO_PASSPHRASE ?? "hotdogbowl";
  const valid = passphrase.trim().toLowerCase() === secret.toLowerCase();

  if (!valid) {
    return NextResponse.json({ valid: false }, { status: 401 });
  }

  const response = NextResponse.json({ valid: true });
  response.cookies.set("bnto-access", "granted", {
    path: "/",
    maxAge: 60 * 60 * 24 * 30, // 30 days
    httpOnly: false,
  });

  return response;
}
