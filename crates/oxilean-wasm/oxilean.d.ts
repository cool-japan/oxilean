/**
 * @cooljapan/oxilean - Lean4-compatible proof assistant for JavaScript/TypeScript
 *
 * Built with wasm-bindgen from Rust.
 */

export interface CheckResult {
  success: boolean;
  declarations: DeclInfo[];
  errors: ErrorInfo[];
  warnings: WarningInfo[];
}

export interface DeclInfo {
  name: string;
  kind: DeclKind;
  ty: string;
}

export type DeclKind =
  | "theorem"
  | "definition"
  | "axiom"
  | "inductive"
  | "structure"
  | "class"
  | "instance"
  | { other: string };

export interface ErrorInfo {
  message: string;
  line: number | null;
  column: number | null;
  source: string | null;
}

export interface WarningInfo {
  message: string;
  line: number | null;
  column: number | null;
}

export interface ReplResult {
  output: string;
  goals: GoalInfo[];
  success: boolean;
  error: string | null;
}

export interface GoalInfo {
  tag: string;
  hypotheses: HypInfo[];
  target: string;
}

export interface HypInfo {
  name: string;
  ty: string;
}

export interface CompletionItem {
  label: string;
  kind: CompletionKind;
  detail: string | null;
  documentation: string | null;
}

export type CompletionKind =
  | "keyword"
  | "function"
  | "theorem"
  | "definition"
  | "variable"
  | "tactic"
  | "snippet";

/**
 * Main OxiLean WASM instance
 */
export class WasmOxiLean {
  constructor();

  /** Check OxiLean source code */
  check(source: string): CheckResult;

  /** Execute a REPL command */
  repl(input: string): ReplResult;

  /** Get completions at a position */
  completions(source: string, line: number, col: number): CompletionItem[];

  /** Get hover info at a position */
  hoverInfo(source: string, line: number, col: number): string | null;

  /** Format OxiLean source code */
  format(source: string): string;

  /** Get session ID */
  readonly sessionId: string;

  /** Get REPL history */
  history(): string[];

  /** Clear REPL history */
  clearHistory(): void;

  /** Get OxiLean version */
  static version(): string;
}

/** Quick check source without creating an instance */
export function checkSource(source: string): CheckResult;

/** Get OxiLean version */
export function getVersion(): string;
