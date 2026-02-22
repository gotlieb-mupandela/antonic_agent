import { execSync } from "node:child_process";

const isVercel = Boolean(process.env.VERCEL);
const command = isVercel
  ? "pnpm --dir services/antonic-agent-share run build"
  : "pnpm --filter @Apnium Technology/antonic-agent build";

execSync(command, { stdio: "inherit" });
