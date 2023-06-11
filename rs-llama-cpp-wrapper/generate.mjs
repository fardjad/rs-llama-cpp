import Parser from "tree-sitter";
import Cpp from "tree-sitter-cpp";
import fs from "node:fs";

const parser = new Parser();
parser.setLanguage(Cpp);

const sourceCode = fs.readFileSync(
  new URL("llama.cpp/examples/main/main.cpp", import.meta.url),
  "utf8"
);
const tree = parser.parse(sourceCode);

const getLineIndex = (query) => {
  const parserQuery = new Parser.Query(Cpp, query);
  const matches = parserQuery
    .matches(tree.rootNode)
    .filter(({ captures }) => captures.length > 0);
  const { startPosition, endPosition } = matches
    .at(0)
    .captures.filter(({ name }) => name === "node")
    .at(0).node;

  return [startPosition.row, endPosition.row];
};

const lines = sourceCode.split("\n");

// replace from bottom to top to avoid index changes

// display text
const displayText = `((comment) @comment (#match? @comment " display text") (if_statement) @node)`;
const [displayTextStart, displayTextEnd] = getLineIndex(displayText);
lines.splice(
  displayTextStart,
  displayTextEnd - displayTextStart + 1,
  ...[
    `        if (input_echo) {`,
    `            if (on_token != nullptr) {`,
    `                for (auto id : embd) {`,
    `                    on_token(llama_token_to_str(ctx, id));`,
    `                }`,
    `            }`,
    `        }`,
  ]
);

// input echo
const inputEcho = `(declaration type: (primitive_type) declarator: (init_declarator declarator: (identifier) @identifier (#match? @identifier "input_echo") value: (true))) @node`;
const [inputEchoStart, inputEchoEnd] = getLineIndex(inputEcho);
lines.splice(
  inputEchoStart,
  inputEchoEnd - inputEchoStart + 1,
  ...[`    bool input_echo = false;`]
);

// if params parse
const ifParamsParse = `(if_statement condition: (condition_clause value: (binary_expression left: (call_expression function: (identifier) @identifier (#match? @identifier "gpt_params_parse") arguments: (argument_list (identifier) (identifier) (identifier))) right: (false))) consequence: (compound_statement (return_statement (number_literal)))) @node`;
const [ifParamsStart, ifParamsEnd] = getLineIndex(ifParamsParse);
lines.splice(ifParamsStart, ifParamsEnd - ifParamsStart + 1);

// params
const paramsDeclaration = `(declaration type: (type_identifier) @type-identifier (#match? @type-identifier "gpt_params") declarator: (identifier) @identifier (#match? @identifier "params")) @node`;
const [paramsStart, paramsEnd] = getLineIndex(paramsDeclaration);
lines.splice(paramsStart, paramsEnd - paramsStart + 1);

// main
const mainFunction = `(function_definition declarator: (function_declarator declarator: (identifier) @identifier-name) (#match? @identifier-name "main")) @node`;
const [mainStart] = getLineIndex(mainFunction);
lines.splice(mainStart, 1, [
  `int run_inference(gpt_params params, token_callback on_token = nullptr) {`,
]);

// includes
const includeBuildInfo = `(preproc_include path: (string_literal) @file (#match? @file "build-info.h")) @node`;
const [includeBuildInfoStart] = getLineIndex(includeBuildInfo);
lines.splice(includeBuildInfoStart, 1, ...[`#include "run-inference.h"`]);
const includeLlama = `(preproc_include path: (string_literal) @file (#match? @file "llama.h")) @node`;
const [includeLlamaStart] = getLineIndex(includeLlama);
lines.splice(includeLlamaStart, 1);
const includeCommon = `(preproc_include path: (string_literal) @file (#match? @file "common.h")) @node`;
const [includeCommonStart] = getLineIndex(includeCommon);
lines.splice(includeCommonStart, 1);

fs.writeFileSync(
  new URL("run-inference.cpp", import.meta.url),
  lines.join("\n"),
  "utf8"
);
