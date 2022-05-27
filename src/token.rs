

enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, COMMA,
    DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, GREATER,
    GREATER_EQUAL, LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, ELSE, FALSE, FN, FOR, IF,
    NIL, OR, PRINT, RETURN, TRUE, VAR,
    WHILE, MAP, FOLD

    EOF
}


enum LiteralType {
    Int,
    Str,
    Identifier
}


pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: LiteralType,
    line: u32
}


