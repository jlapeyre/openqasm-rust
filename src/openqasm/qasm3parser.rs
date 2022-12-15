// Generated from ./openqasm/qasm3Parser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::qasm3parserlistener::*;
use super::qasm3parservisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const OPENQASM:isize=1; 
		pub const INCLUDE:isize=2; 
		pub const DEFCALGRAMMAR:isize=3; 
		pub const DEF:isize=4; 
		pub const CAL:isize=5; 
		pub const DEFCAL:isize=6; 
		pub const GATE:isize=7; 
		pub const EXTERN:isize=8; 
		pub const BOX:isize=9; 
		pub const LET:isize=10; 
		pub const BREAK:isize=11; 
		pub const CONTINUE:isize=12; 
		pub const IF:isize=13; 
		pub const ELSE:isize=14; 
		pub const END:isize=15; 
		pub const RETURN:isize=16; 
		pub const FOR:isize=17; 
		pub const WHILE:isize=18; 
		pub const IN:isize=19; 
		pub const PRAGMA:isize=20; 
		pub const AnnotationKeyword:isize=21; 
		pub const INPUT:isize=22; 
		pub const OUTPUT:isize=23; 
		pub const CONST:isize=24; 
		pub const READONLY:isize=25; 
		pub const MUTABLE:isize=26; 
		pub const QREG:isize=27; 
		pub const QUBIT:isize=28; 
		pub const CREG:isize=29; 
		pub const BOOL:isize=30; 
		pub const BIT:isize=31; 
		pub const INT:isize=32; 
		pub const UINT:isize=33; 
		pub const FLOAT:isize=34; 
		pub const ANGLE:isize=35; 
		pub const COMPLEX:isize=36; 
		pub const ARRAY:isize=37; 
		pub const VOID:isize=38; 
		pub const DURATION:isize=39; 
		pub const STRETCH:isize=40; 
		pub const GPHASE:isize=41; 
		pub const INV:isize=42; 
		pub const POW:isize=43; 
		pub const CTRL:isize=44; 
		pub const NEGCTRL:isize=45; 
		pub const DIM:isize=46; 
		pub const DURATIONOF:isize=47; 
		pub const DELAY:isize=48; 
		pub const RESET:isize=49; 
		pub const MEASURE:isize=50; 
		pub const BARRIER:isize=51; 
		pub const BooleanLiteral:isize=52; 
		pub const LBRACKET:isize=53; 
		pub const RBRACKET:isize=54; 
		pub const LBRACE:isize=55; 
		pub const RBRACE:isize=56; 
		pub const LPAREN:isize=57; 
		pub const RPAREN:isize=58; 
		pub const COLON:isize=59; 
		pub const SEMICOLON:isize=60; 
		pub const DOT:isize=61; 
		pub const COMMA:isize=62; 
		pub const EQUALS:isize=63; 
		pub const ARROW:isize=64; 
		pub const PLUS:isize=65; 
		pub const DOUBLE_PLUS:isize=66; 
		pub const MINUS:isize=67; 
		pub const ASTERISK:isize=68; 
		pub const DOUBLE_ASTERISK:isize=69; 
		pub const SLASH:isize=70; 
		pub const PERCENT:isize=71; 
		pub const PIPE:isize=72; 
		pub const DOUBLE_PIPE:isize=73; 
		pub const AMPERSAND:isize=74; 
		pub const DOUBLE_AMPERSAND:isize=75; 
		pub const CARET:isize=76; 
		pub const AT:isize=77; 
		pub const TILDE:isize=78; 
		pub const EXCLAMATION_POINT:isize=79; 
		pub const EqualityOperator:isize=80; 
		pub const CompoundAssignmentOperator:isize=81; 
		pub const ComparisonOperator:isize=82; 
		pub const BitshiftOperator:isize=83; 
		pub const IMAG:isize=84; 
		pub const ImaginaryLiteral:isize=85; 
		pub const BinaryIntegerLiteral:isize=86; 
		pub const OctalIntegerLiteral:isize=87; 
		pub const DecimalIntegerLiteral:isize=88; 
		pub const HexIntegerLiteral:isize=89; 
		pub const Identifier:isize=90; 
		pub const HardwareQubit:isize=91; 
		pub const FloatLiteral:isize=92; 
		pub const TimingLiteral:isize=93; 
		pub const BitstringLiteral:isize=94; 
		pub const StringLiteral:isize=95; 
		pub const Whitespace:isize=96; 
		pub const Newline:isize=97; 
		pub const LineComment:isize=98; 
		pub const BlockComment:isize=99; 
		pub const VERSION_IDENTIFER_WHITESPACE:isize=100; 
		pub const VersionSpecifier:isize=101; 
		pub const EAT_INITIAL_SPACE:isize=102; 
		pub const EAT_LINE_END:isize=103; 
		pub const RemainingLineContent:isize=104; 
		pub const CAL_PRELUDE_WHITESPACE:isize=105; 
		pub const CAL_PRELUDE_COMMENT:isize=106; 
		pub const DEFCAL_PRELUDE_WHITESPACE:isize=107; 
		pub const DEFCAL_PRELUDE_COMMENT:isize=108; 
		pub const CalibrationBlock:isize=109;
	pub const RULE_program:usize = 0; 
	pub const RULE_version:usize = 1; 
	pub const RULE_statement:usize = 2; 
	pub const RULE_annotation:usize = 3; 
	pub const RULE_scope:usize = 4; 
	pub const RULE_pragma:usize = 5; 
	pub const RULE_statementOrScope:usize = 6; 
	pub const RULE_calibrationGrammarStatement:usize = 7; 
	pub const RULE_includeStatement:usize = 8; 
	pub const RULE_breakStatement:usize = 9; 
	pub const RULE_continueStatement:usize = 10; 
	pub const RULE_endStatement:usize = 11; 
	pub const RULE_forStatement:usize = 12; 
	pub const RULE_ifStatement:usize = 13; 
	pub const RULE_returnStatement:usize = 14; 
	pub const RULE_whileStatement:usize = 15; 
	pub const RULE_barrierStatement:usize = 16; 
	pub const RULE_boxStatement:usize = 17; 
	pub const RULE_delayStatement:usize = 18; 
	pub const RULE_gateCallStatement:usize = 19; 
	pub const RULE_measureArrowAssignmentStatement:usize = 20; 
	pub const RULE_resetStatement:usize = 21; 
	pub const RULE_aliasDeclarationStatement:usize = 22; 
	pub const RULE_classicalDeclarationStatement:usize = 23; 
	pub const RULE_constDeclarationStatement:usize = 24; 
	pub const RULE_ioDeclarationStatement:usize = 25; 
	pub const RULE_oldStyleDeclarationStatement:usize = 26; 
	pub const RULE_quantumDeclarationStatement:usize = 27; 
	pub const RULE_defStatement:usize = 28; 
	pub const RULE_externStatement:usize = 29; 
	pub const RULE_gateStatement:usize = 30; 
	pub const RULE_assignmentStatement:usize = 31; 
	pub const RULE_expressionStatement:usize = 32; 
	pub const RULE_calStatement:usize = 33; 
	pub const RULE_defcalStatement:usize = 34; 
	pub const RULE_expression:usize = 35; 
	pub const RULE_aliasExpression:usize = 36; 
	pub const RULE_declarationExpression:usize = 37; 
	pub const RULE_measureExpression:usize = 38; 
	pub const RULE_rangeExpression:usize = 39; 
	pub const RULE_setExpression:usize = 40; 
	pub const RULE_arrayLiteral:usize = 41; 
	pub const RULE_indexOperator:usize = 42; 
	pub const RULE_indexedIdentifier:usize = 43; 
	pub const RULE_returnSignature:usize = 44; 
	pub const RULE_gateModifier:usize = 45; 
	pub const RULE_scalarType:usize = 46; 
	pub const RULE_qubitType:usize = 47; 
	pub const RULE_arrayType:usize = 48; 
	pub const RULE_arrayReferenceType:usize = 49; 
	pub const RULE_designator:usize = 50; 
	pub const RULE_defcalTarget:usize = 51; 
	pub const RULE_defcalArgumentDefinition:usize = 52; 
	pub const RULE_defcalOperand:usize = 53; 
	pub const RULE_gateOperand:usize = 54; 
	pub const RULE_externArgument:usize = 55; 
	pub const RULE_argumentDefinition:usize = 56; 
	pub const RULE_argumentDefinitionList:usize = 57; 
	pub const RULE_defcalArgumentDefinitionList:usize = 58; 
	pub const RULE_defcalOperandList:usize = 59; 
	pub const RULE_expressionList:usize = 60; 
	pub const RULE_identifierList:usize = 61; 
	pub const RULE_gateOperandList:usize = 62; 
	pub const RULE_externArgumentList:usize = 63;
	pub const ruleNames: [&'static str; 64] =  [
		"program", "version", "statement", "annotation", "scope", "pragma", "statementOrScope", 
		"calibrationGrammarStatement", "includeStatement", "breakStatement", "continueStatement", 
		"endStatement", "forStatement", "ifStatement", "returnStatement", "whileStatement", 
		"barrierStatement", "boxStatement", "delayStatement", "gateCallStatement", 
		"measureArrowAssignmentStatement", "resetStatement", "aliasDeclarationStatement", 
		"classicalDeclarationStatement", "constDeclarationStatement", "ioDeclarationStatement", 
		"oldStyleDeclarationStatement", "quantumDeclarationStatement", "defStatement", 
		"externStatement", "gateStatement", "assignmentStatement", "expressionStatement", 
		"calStatement", "defcalStatement", "expression", "aliasExpression", "declarationExpression", 
		"measureExpression", "rangeExpression", "setExpression", "arrayLiteral", 
		"indexOperator", "indexedIdentifier", "returnSignature", "gateModifier", 
		"scalarType", "qubitType", "arrayType", "arrayReferenceType", "designator", 
		"defcalTarget", "defcalArgumentDefinition", "defcalOperand", "gateOperand", 
		"externArgument", "argumentDefinition", "argumentDefinitionList", "defcalArgumentDefinitionList", 
		"defcalOperandList", "expressionList", "identifierList", "gateOperandList", 
		"externArgumentList"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;85] = [
		None, Some("'OPENQASM'"), Some("'include'"), Some("'defcalgrammar'"), 
		Some("'def'"), Some("'cal'"), Some("'defcal'"), Some("'gate'"), Some("'extern'"), 
		Some("'box'"), Some("'let'"), Some("'break'"), Some("'continue'"), Some("'if'"), 
		Some("'else'"), Some("'end'"), Some("'return'"), Some("'for'"), Some("'while'"), 
		Some("'in'"), None, None, Some("'input'"), Some("'output'"), Some("'const'"), 
		Some("'readonly'"), Some("'mutable'"), Some("'qreg'"), Some("'qubit'"), 
		Some("'creg'"), Some("'bool'"), Some("'bit'"), Some("'int'"), Some("'uint'"), 
		Some("'float'"), Some("'angle'"), Some("'complex'"), Some("'array'"), 
		Some("'void'"), Some("'duration'"), Some("'stretch'"), Some("'gphase'"), 
		Some("'inv'"), Some("'pow'"), Some("'ctrl'"), Some("'negctrl'"), Some("'#dim'"), 
		Some("'durationof'"), Some("'delay'"), Some("'reset'"), Some("'measure'"), 
		Some("'barrier'"), None, Some("'['"), Some("']'"), Some("'{'"), Some("'}'"), 
		Some("'('"), Some("')'"), Some("':'"), Some("';'"), Some("'.'"), Some("','"), 
		Some("'='"), Some("'->'"), Some("'+'"), Some("'++'"), Some("'-'"), Some("'*'"), 
		Some("'**'"), Some("'/'"), Some("'%'"), Some("'|'"), Some("'||'"), Some("'&'"), 
		Some("'&&'"), Some("'^'"), Some("'@'"), Some("'~'"), Some("'!'"), None, 
		None, None, None, Some("'im'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;110]  = [
		None, Some("OPENQASM"), Some("INCLUDE"), Some("DEFCALGRAMMAR"), Some("DEF"), 
		Some("CAL"), Some("DEFCAL"), Some("GATE"), Some("EXTERN"), Some("BOX"), 
		Some("LET"), Some("BREAK"), Some("CONTINUE"), Some("IF"), Some("ELSE"), 
		Some("END"), Some("RETURN"), Some("FOR"), Some("WHILE"), Some("IN"), Some("PRAGMA"), 
		Some("AnnotationKeyword"), Some("INPUT"), Some("OUTPUT"), Some("CONST"), 
		Some("READONLY"), Some("MUTABLE"), Some("QREG"), Some("QUBIT"), Some("CREG"), 
		Some("BOOL"), Some("BIT"), Some("INT"), Some("UINT"), Some("FLOAT"), Some("ANGLE"), 
		Some("COMPLEX"), Some("ARRAY"), Some("VOID"), Some("DURATION"), Some("STRETCH"), 
		Some("GPHASE"), Some("INV"), Some("POW"), Some("CTRL"), Some("NEGCTRL"), 
		Some("DIM"), Some("DURATIONOF"), Some("DELAY"), Some("RESET"), Some("MEASURE"), 
		Some("BARRIER"), Some("BooleanLiteral"), Some("LBRACKET"), Some("RBRACKET"), 
		Some("LBRACE"), Some("RBRACE"), Some("LPAREN"), Some("RPAREN"), Some("COLON"), 
		Some("SEMICOLON"), Some("DOT"), Some("COMMA"), Some("EQUALS"), Some("ARROW"), 
		Some("PLUS"), Some("DOUBLE_PLUS"), Some("MINUS"), Some("ASTERISK"), Some("DOUBLE_ASTERISK"), 
		Some("SLASH"), Some("PERCENT"), Some("PIPE"), Some("DOUBLE_PIPE"), Some("AMPERSAND"), 
		Some("DOUBLE_AMPERSAND"), Some("CARET"), Some("AT"), Some("TILDE"), Some("EXCLAMATION_POINT"), 
		Some("EqualityOperator"), Some("CompoundAssignmentOperator"), Some("ComparisonOperator"), 
		Some("BitshiftOperator"), Some("IMAG"), Some("ImaginaryLiteral"), Some("BinaryIntegerLiteral"), 
		Some("OctalIntegerLiteral"), Some("DecimalIntegerLiteral"), Some("HexIntegerLiteral"), 
		Some("Identifier"), Some("HardwareQubit"), Some("FloatLiteral"), Some("TimingLiteral"), 
		Some("BitstringLiteral"), Some("StringLiteral"), Some("Whitespace"), Some("Newline"), 
		Some("LineComment"), Some("BlockComment"), Some("VERSION_IDENTIFER_WHITESPACE"), 
		Some("VersionSpecifier"), Some("EAT_INITIAL_SPACE"), Some("EAT_LINE_END"), 
		Some("RemainingLineContent"), Some("CAL_PRELUDE_WHITESPACE"), Some("CAL_PRELUDE_COMMENT"), 
		Some("DEFCAL_PRELUDE_WHITESPACE"), Some("DEFCAL_PRELUDE_COMMENT"), Some("CalibrationBlock")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,qasm3ParserExt<'input>, I, qasm3ParserContextType , dyn qasm3ParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type qasm3ParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, qasm3ParserContextType , dyn qasm3ParserListener<'input> + 'a>;

/// Parser for qasm3Parser grammar
pub struct qasm3Parser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				qasm3ParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> qasm3Parser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> qasm3Parser<'input, I, DefaultErrorStrategy<'input,qasm3ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for qasm3Parser
pub trait qasm3ParserContext<'input>:
	for<'x> Listenable<dyn qasm3ParserListener<'input> + 'x > + 
	for<'x> Visitable<dyn qasm3ParserVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=qasm3ParserContextType>
{}

antlr_rust::coerce_from!{ 'input : qasm3ParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn qasm3ParserContext<'input> + 'input
where
    T: qasm3ParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn qasm3ParserVisitor<'input> + 'x))
    }
}

impl<'input> qasm3ParserContext<'input> for TerminalNode<'input,qasm3ParserContextType> {}
impl<'input> qasm3ParserContext<'input> for ErrorNode<'input,qasm3ParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn qasm3ParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn qasm3ParserListener<'input> + 'input }

pub struct qasm3ParserContextType;
antlr_rust::tid!{qasm3ParserContextType}

impl<'input> ParserNodeType<'input> for qasm3ParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn qasm3ParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct qasm3ParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> qasm3ParserExt<'input>{
}
antlr_rust::tid! { qasm3ParserExt<'a> }

impl<'input> TokenAware<'input> for qasm3ParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for qasm3ParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for qasm3ParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "qasm3Parser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn qasm3ParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					35 => qasm3Parser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> qasm3Parser<'input, I, DefaultErrorStrategy<'input,qasm3ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 16)
				}
				1=>{
					recog.precpred(None, 14)
				}
				2=>{
					recog.precpred(None, 13)
				}
				3=>{
					recog.precpred(None, 12)
				}
				4=>{
					recog.precpred(None, 11)
				}
				5=>{
					recog.precpred(None, 10)
				}
				6=>{
					recog.precpred(None, 9)
				}
				7=>{
					recog.precpred(None, 8)
				}
				8=>{
					recog.precpred(None, 7)
				}
				9=>{
					recog.precpred(None, 6)
				}
				10=>{
					recog.precpred(None, 5)
				}
				11=>{
					recog.precpred(None, 17)
				}
			_ => true
		}
	}
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn version(&self) -> Option<Rc<VersionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(129);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==OPENQASM {
				{
				/*InvokeRule version*/
				recog.base.set_state(128);
				recog.version()?;

				}
			}

			recog.base.set_state(134);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << INCLUDE) | (1usize << DEFCALGRAMMAR) | (1usize << DEF) | (1usize << CAL) | (1usize << DEFCAL) | (1usize << GATE) | (1usize << EXTERN) | (1usize << BOX) | (1usize << LET) | (1usize << BREAK) | (1usize << CONTINUE) | (1usize << IF) | (1usize << END) | (1usize << RETURN) | (1usize << FOR) | (1usize << WHILE) | (1usize << PRAGMA) | (1usize << AnnotationKeyword) | (1usize << INPUT) | (1usize << OUTPUT) | (1usize << CONST) | (1usize << QREG) | (1usize << QUBIT) | (1usize << CREG) | (1usize << BOOL) | (1usize << BIT))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (INT - 32)) | (1usize << (UINT - 32)) | (1usize << (FLOAT - 32)) | (1usize << (ANGLE - 32)) | (1usize << (COMPLEX - 32)) | (1usize << (ARRAY - 32)) | (1usize << (DURATION - 32)) | (1usize << (STRETCH - 32)) | (1usize << (GPHASE - 32)) | (1usize << (INV - 32)) | (1usize << (POW - 32)) | (1usize << (CTRL - 32)) | (1usize << (NEGCTRL - 32)) | (1usize << (DURATIONOF - 32)) | (1usize << (DELAY - 32)) | (1usize << (RESET - 32)) | (1usize << (MEASURE - 32)) | (1usize << (BARRIER - 32)) | (1usize << (BooleanLiteral - 32)) | (1usize << (LPAREN - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(131);
				recog.statement()?;

				}
				}
				recog.base.set_state(136);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(137);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- version ----------------
pub type VersionContextAll<'input> = VersionContext<'input>;


pub type VersionContext<'input> = BaseParserRuleContext<'input,VersionContextExt<'input>>;

#[derive(Clone)]
pub struct VersionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for VersionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for VersionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_version(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_version(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for VersionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_version(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_version }
	//fn type_rule_index() -> usize where Self: Sized { RULE_version }
}
antlr_rust::tid!{VersionContextExt<'a>}

impl<'input> VersionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<VersionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPENQASM
/// Returns `None` if there is no child corresponding to token OPENQASM
fn OPENQASM(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(OPENQASM, 0)
}
/// Retrieves first TerminalNode corresponding to token VersionSpecifier
/// Returns `None` if there is no child corresponding to token VersionSpecifier
fn VersionSpecifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(VersionSpecifier, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> VersionContextAttrs<'input> for VersionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn version(&mut self,)
	-> Result<Rc<VersionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_version);
        let mut _localctx: Rc<VersionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(139);
			recog.base.match_token(OPENQASM,&mut recog.err_handler)?;

			recog.base.set_state(140);
			recog.base.match_token(VersionSpecifier,&mut recog.err_handler)?;

			recog.base.set_state(141);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn pragma(&self) -> Option<Rc<PragmaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn aliasDeclarationStatement(&self) -> Option<Rc<AliasDeclarationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentStatement(&self) -> Option<Rc<AssignmentStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn barrierStatement(&self) -> Option<Rc<BarrierStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn boxStatement(&self) -> Option<Rc<BoxStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn breakStatement(&self) -> Option<Rc<BreakStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn calStatement(&self) -> Option<Rc<CalStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn calibrationGrammarStatement(&self) -> Option<Rc<CalibrationGrammarStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classicalDeclarationStatement(&self) -> Option<Rc<ClassicalDeclarationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constDeclarationStatement(&self) -> Option<Rc<ConstDeclarationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn continueStatement(&self) -> Option<Rc<ContinueStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defStatement(&self) -> Option<Rc<DefStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defcalStatement(&self) -> Option<Rc<DefcalStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn delayStatement(&self) -> Option<Rc<DelayStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn endStatement(&self) -> Option<Rc<EndStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionStatement(&self) -> Option<Rc<ExpressionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn externStatement(&self) -> Option<Rc<ExternStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forStatement(&self) -> Option<Rc<ForStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn gateCallStatement(&self) -> Option<Rc<GateCallStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn gateStatement(&self) -> Option<Rc<GateStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ifStatement(&self) -> Option<Rc<IfStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn includeStatement(&self) -> Option<Rc<IncludeStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ioDeclarationStatement(&self) -> Option<Rc<IoDeclarationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn measureArrowAssignmentStatement(&self) -> Option<Rc<MeasureArrowAssignmentStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oldStyleDeclarationStatement(&self) -> Option<Rc<OldStyleDeclarationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn quantumDeclarationStatement(&self) -> Option<Rc<QuantumDeclarationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn resetStatement(&self) -> Option<Rc<ResetStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnStatement(&self) -> Option<Rc<ReturnStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn whileStatement(&self) -> Option<Rc<WhileStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(180);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 PRAGMA 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule pragma*/
					recog.base.set_state(143);
					recog.pragma()?;

					}
				}

			 INCLUDE | DEFCALGRAMMAR | DEF | CAL | DEFCAL | GATE | EXTERN | BOX |
			 LET | BREAK | CONTINUE | IF | END | RETURN | FOR | WHILE | AnnotationKeyword |
			 INPUT | OUTPUT | CONST | QREG | QUBIT | CREG | BOOL | BIT | INT | UINT |
			 FLOAT | ANGLE | COMPLEX | ARRAY | DURATION | STRETCH | GPHASE | INV |
			 POW | CTRL | NEGCTRL | DURATIONOF | DELAY | RESET | MEASURE | BARRIER |
			 BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT | ImaginaryLiteral |
			 BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
			 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
			 BitstringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(147);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==AnnotationKeyword {
						{
						{
						/*InvokeRule annotation*/
						recog.base.set_state(144);
						recog.annotation()?;

						}
						}
						recog.base.set_state(149);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(178);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule aliasDeclarationStatement*/
							recog.base.set_state(150);
							recog.aliasDeclarationStatement()?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule assignmentStatement*/
							recog.base.set_state(151);
							recog.assignmentStatement()?;

							}
						}
					,
						3 =>{
							{
							/*InvokeRule barrierStatement*/
							recog.base.set_state(152);
							recog.barrierStatement()?;

							}
						}
					,
						4 =>{
							{
							/*InvokeRule boxStatement*/
							recog.base.set_state(153);
							recog.boxStatement()?;

							}
						}
					,
						5 =>{
							{
							/*InvokeRule breakStatement*/
							recog.base.set_state(154);
							recog.breakStatement()?;

							}
						}
					,
						6 =>{
							{
							/*InvokeRule calStatement*/
							recog.base.set_state(155);
							recog.calStatement()?;

							}
						}
					,
						7 =>{
							{
							/*InvokeRule calibrationGrammarStatement*/
							recog.base.set_state(156);
							recog.calibrationGrammarStatement()?;

							}
						}
					,
						8 =>{
							{
							/*InvokeRule classicalDeclarationStatement*/
							recog.base.set_state(157);
							recog.classicalDeclarationStatement()?;

							}
						}
					,
						9 =>{
							{
							/*InvokeRule constDeclarationStatement*/
							recog.base.set_state(158);
							recog.constDeclarationStatement()?;

							}
						}
					,
						10 =>{
							{
							/*InvokeRule continueStatement*/
							recog.base.set_state(159);
							recog.continueStatement()?;

							}
						}
					,
						11 =>{
							{
							/*InvokeRule defStatement*/
							recog.base.set_state(160);
							recog.defStatement()?;

							}
						}
					,
						12 =>{
							{
							/*InvokeRule defcalStatement*/
							recog.base.set_state(161);
							recog.defcalStatement()?;

							}
						}
					,
						13 =>{
							{
							/*InvokeRule delayStatement*/
							recog.base.set_state(162);
							recog.delayStatement()?;

							}
						}
					,
						14 =>{
							{
							/*InvokeRule endStatement*/
							recog.base.set_state(163);
							recog.endStatement()?;

							}
						}
					,
						15 =>{
							{
							/*InvokeRule expressionStatement*/
							recog.base.set_state(164);
							recog.expressionStatement()?;

							}
						}
					,
						16 =>{
							{
							/*InvokeRule externStatement*/
							recog.base.set_state(165);
							recog.externStatement()?;

							}
						}
					,
						17 =>{
							{
							/*InvokeRule forStatement*/
							recog.base.set_state(166);
							recog.forStatement()?;

							}
						}
					,
						18 =>{
							{
							/*InvokeRule gateCallStatement*/
							recog.base.set_state(167);
							recog.gateCallStatement()?;

							}
						}
					,
						19 =>{
							{
							/*InvokeRule gateStatement*/
							recog.base.set_state(168);
							recog.gateStatement()?;

							}
						}
					,
						20 =>{
							{
							/*InvokeRule ifStatement*/
							recog.base.set_state(169);
							recog.ifStatement()?;

							}
						}
					,
						21 =>{
							{
							/*InvokeRule includeStatement*/
							recog.base.set_state(170);
							recog.includeStatement()?;

							}
						}
					,
						22 =>{
							{
							/*InvokeRule ioDeclarationStatement*/
							recog.base.set_state(171);
							recog.ioDeclarationStatement()?;

							}
						}
					,
						23 =>{
							{
							/*InvokeRule measureArrowAssignmentStatement*/
							recog.base.set_state(172);
							recog.measureArrowAssignmentStatement()?;

							}
						}
					,
						24 =>{
							{
							/*InvokeRule oldStyleDeclarationStatement*/
							recog.base.set_state(173);
							recog.oldStyleDeclarationStatement()?;

							}
						}
					,
						25 =>{
							{
							/*InvokeRule quantumDeclarationStatement*/
							recog.base.set_state(174);
							recog.quantumDeclarationStatement()?;

							}
						}
					,
						26 =>{
							{
							/*InvokeRule resetStatement*/
							recog.base.set_state(175);
							recog.resetStatement()?;

							}
						}
					,
						27 =>{
							{
							/*InvokeRule returnStatement*/
							recog.base.set_state(176);
							recog.returnStatement()?;

							}
						}
					,
						28 =>{
							{
							/*InvokeRule whileStatement*/
							recog.base.set_state(177);
							recog.whileStatement()?;

							}
						}

						_ => {}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotation ----------------
pub type AnnotationContextAll<'input> = AnnotationContext<'input>;


pub type AnnotationContext<'input> = BaseParserRuleContext<'input,AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for AnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for AnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotation(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_annotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for AnnotationContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_annotation(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotation }
}
antlr_rust::tid!{AnnotationContextExt<'a>}

impl<'input> AnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<AnnotationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AnnotationKeyword
/// Returns `None` if there is no child corresponding to token AnnotationKeyword
fn AnnotationKeyword(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(AnnotationKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token RemainingLineContent
/// Returns `None` if there is no child corresponding to token RemainingLineContent
fn RemainingLineContent(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RemainingLineContent, 0)
}

}

impl<'input> AnnotationContextAttrs<'input> for AnnotationContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotation(&mut self,)
	-> Result<Rc<AnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_annotation);
        let mut _localctx: Rc<AnnotationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(182);
			recog.base.match_token(AnnotationKeyword,&mut recog.err_handler)?;

			recog.base.set_state(184);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==RemainingLineContent {
				{
				recog.base.set_state(183);
				recog.base.match_token(RemainingLineContent,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scope ----------------
pub type ScopeContextAll<'input> = ScopeContext<'input>;


pub type ScopeContext<'input> = BaseParserRuleContext<'input,ScopeContextExt<'input>>;

#[derive(Clone)]
pub struct ScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ScopeContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_scope(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_scope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_scope(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scope }
}
antlr_rust::tid!{ScopeContextExt<'a>}

impl<'input> ScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScopeContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ScopeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ScopeContextAttrs<'input> for ScopeContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scope(&mut self,)
	-> Result<Rc<ScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_scope);
        let mut _localctx: Rc<ScopeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(186);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(190);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << INCLUDE) | (1usize << DEFCALGRAMMAR) | (1usize << DEF) | (1usize << CAL) | (1usize << DEFCAL) | (1usize << GATE) | (1usize << EXTERN) | (1usize << BOX) | (1usize << LET) | (1usize << BREAK) | (1usize << CONTINUE) | (1usize << IF) | (1usize << END) | (1usize << RETURN) | (1usize << FOR) | (1usize << WHILE) | (1usize << PRAGMA) | (1usize << AnnotationKeyword) | (1usize << INPUT) | (1usize << OUTPUT) | (1usize << CONST) | (1usize << QREG) | (1usize << QUBIT) | (1usize << CREG) | (1usize << BOOL) | (1usize << BIT))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (INT - 32)) | (1usize << (UINT - 32)) | (1usize << (FLOAT - 32)) | (1usize << (ANGLE - 32)) | (1usize << (COMPLEX - 32)) | (1usize << (ARRAY - 32)) | (1usize << (DURATION - 32)) | (1usize << (STRETCH - 32)) | (1usize << (GPHASE - 32)) | (1usize << (INV - 32)) | (1usize << (POW - 32)) | (1usize << (CTRL - 32)) | (1usize << (NEGCTRL - 32)) | (1usize << (DURATIONOF - 32)) | (1usize << (DELAY - 32)) | (1usize << (RESET - 32)) | (1usize << (MEASURE - 32)) | (1usize << (BARRIER - 32)) | (1usize << (BooleanLiteral - 32)) | (1usize << (LPAREN - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(187);
				recog.statement()?;

				}
				}
				recog.base.set_state(192);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(193);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pragma ----------------
pub type PragmaContextAll<'input> = PragmaContext<'input>;


pub type PragmaContext<'input> = BaseParserRuleContext<'input,PragmaContextExt<'input>>;

#[derive(Clone)]
pub struct PragmaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for PragmaContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for PragmaContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pragma(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_pragma(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for PragmaContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_pragma(self);
	}
}

impl<'input> CustomRuleContext<'input> for PragmaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pragma }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pragma }
}
antlr_rust::tid!{PragmaContextExt<'a>}

impl<'input> PragmaContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PragmaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PragmaContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PragmaContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<PragmaContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PRAGMA
/// Returns `None` if there is no child corresponding to token PRAGMA
fn PRAGMA(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(PRAGMA, 0)
}
/// Retrieves first TerminalNode corresponding to token RemainingLineContent
/// Returns `None` if there is no child corresponding to token RemainingLineContent
fn RemainingLineContent(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RemainingLineContent, 0)
}

}

impl<'input> PragmaContextAttrs<'input> for PragmaContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pragma(&mut self,)
	-> Result<Rc<PragmaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PragmaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_pragma);
        let mut _localctx: Rc<PragmaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(195);
			recog.base.match_token(PRAGMA,&mut recog.err_handler)?;

			recog.base.set_state(196);
			recog.base.match_token(RemainingLineContent,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statementOrScope ----------------
pub type StatementOrScopeContextAll<'input> = StatementOrScopeContext<'input>;


pub type StatementOrScopeContext<'input> = BaseParserRuleContext<'input,StatementOrScopeContextExt<'input>>;

#[derive(Clone)]
pub struct StatementOrScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for StatementOrScopeContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for StatementOrScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statementOrScope(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_statementOrScope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for StatementOrScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_statementOrScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementOrScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statementOrScope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statementOrScope }
}
antlr_rust::tid!{StatementOrScopeContextExt<'a>}

impl<'input> StatementOrScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementOrScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementOrScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementOrScopeContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<StatementOrScopeContextExt<'input>>{

fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementOrScopeContextAttrs<'input> for StatementOrScopeContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statementOrScope(&mut self,)
	-> Result<Rc<StatementOrScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementOrScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_statementOrScope);
        let mut _localctx: Rc<StatementOrScopeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(200);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INCLUDE | DEFCALGRAMMAR | DEF | CAL | DEFCAL | GATE | EXTERN | BOX |
			 LET | BREAK | CONTINUE | IF | END | RETURN | FOR | WHILE | PRAGMA | AnnotationKeyword |
			 INPUT | OUTPUT | CONST | QREG | QUBIT | CREG | BOOL | BIT | INT | UINT |
			 FLOAT | ANGLE | COMPLEX | ARRAY | DURATION | STRETCH | GPHASE | INV |
			 POW | CTRL | NEGCTRL | DURATIONOF | DELAY | RESET | MEASURE | BARRIER |
			 BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT | ImaginaryLiteral |
			 BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
			 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
			 BitstringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule statement*/
					recog.base.set_state(198);
					recog.statement()?;

					}
				}

			 LBRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule scope*/
					recog.base.set_state(199);
					recog.scope()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- calibrationGrammarStatement ----------------
pub type CalibrationGrammarStatementContextAll<'input> = CalibrationGrammarStatementContext<'input>;


pub type CalibrationGrammarStatementContext<'input> = BaseParserRuleContext<'input,CalibrationGrammarStatementContextExt<'input>>;

#[derive(Clone)]
pub struct CalibrationGrammarStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for CalibrationGrammarStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for CalibrationGrammarStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_calibrationGrammarStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_calibrationGrammarStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for CalibrationGrammarStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_calibrationGrammarStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for CalibrationGrammarStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_calibrationGrammarStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_calibrationGrammarStatement }
}
antlr_rust::tid!{CalibrationGrammarStatementContextExt<'a>}

impl<'input> CalibrationGrammarStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CalibrationGrammarStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CalibrationGrammarStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CalibrationGrammarStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<CalibrationGrammarStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEFCALGRAMMAR
/// Returns `None` if there is no child corresponding to token DEFCALGRAMMAR
fn DEFCALGRAMMAR(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DEFCALGRAMMAR, 0)
}
/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(StringLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> CalibrationGrammarStatementContextAttrs<'input> for CalibrationGrammarStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn calibrationGrammarStatement(&mut self,)
	-> Result<Rc<CalibrationGrammarStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CalibrationGrammarStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_calibrationGrammarStatement);
        let mut _localctx: Rc<CalibrationGrammarStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(202);
			recog.base.match_token(DEFCALGRAMMAR,&mut recog.err_handler)?;

			recog.base.set_state(203);
			recog.base.match_token(StringLiteral,&mut recog.err_handler)?;

			recog.base.set_state(204);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- includeStatement ----------------
pub type IncludeStatementContextAll<'input> = IncludeStatementContext<'input>;


pub type IncludeStatementContext<'input> = BaseParserRuleContext<'input,IncludeStatementContextExt<'input>>;

#[derive(Clone)]
pub struct IncludeStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for IncludeStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for IncludeStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_includeStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_includeStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for IncludeStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_includeStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for IncludeStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_includeStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_includeStatement }
}
antlr_rust::tid!{IncludeStatementContextExt<'a>}

impl<'input> IncludeStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IncludeStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IncludeStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IncludeStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<IncludeStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INCLUDE
/// Returns `None` if there is no child corresponding to token INCLUDE
fn INCLUDE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(INCLUDE, 0)
}
/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(StringLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> IncludeStatementContextAttrs<'input> for IncludeStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn includeStatement(&mut self,)
	-> Result<Rc<IncludeStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IncludeStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_includeStatement);
        let mut _localctx: Rc<IncludeStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(206);
			recog.base.match_token(INCLUDE,&mut recog.err_handler)?;

			recog.base.set_state(207);
			recog.base.match_token(StringLiteral,&mut recog.err_handler)?;

			recog.base.set_state(208);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- breakStatement ----------------
pub type BreakStatementContextAll<'input> = BreakStatementContext<'input>;


pub type BreakStatementContext<'input> = BaseParserRuleContext<'input,BreakStatementContextExt<'input>>;

#[derive(Clone)]
pub struct BreakStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for BreakStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for BreakStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_breakStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_breakStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for BreakStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_breakStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for BreakStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_breakStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_breakStatement }
}
antlr_rust::tid!{BreakStatementContextExt<'a>}

impl<'input> BreakStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BreakStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BreakStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BreakStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<BreakStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BREAK
/// Returns `None` if there is no child corresponding to token BREAK
fn BREAK(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(BREAK, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> BreakStatementContextAttrs<'input> for BreakStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn breakStatement(&mut self,)
	-> Result<Rc<BreakStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BreakStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_breakStatement);
        let mut _localctx: Rc<BreakStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(210);
			recog.base.match_token(BREAK,&mut recog.err_handler)?;

			recog.base.set_state(211);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- continueStatement ----------------
pub type ContinueStatementContextAll<'input> = ContinueStatementContext<'input>;


pub type ContinueStatementContext<'input> = BaseParserRuleContext<'input,ContinueStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ContinueStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ContinueStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ContinueStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_continueStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_continueStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ContinueStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_continueStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ContinueStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_continueStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_continueStatement }
}
antlr_rust::tid!{ContinueStatementContextExt<'a>}

impl<'input> ContinueStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ContinueStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ContinueStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ContinueStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ContinueStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONTINUE
/// Returns `None` if there is no child corresponding to token CONTINUE
fn CONTINUE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CONTINUE, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> ContinueStatementContextAttrs<'input> for ContinueStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn continueStatement(&mut self,)
	-> Result<Rc<ContinueStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ContinueStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_continueStatement);
        let mut _localctx: Rc<ContinueStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(213);
			recog.base.match_token(CONTINUE,&mut recog.err_handler)?;

			recog.base.set_state(214);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- endStatement ----------------
pub type EndStatementContextAll<'input> = EndStatementContext<'input>;


pub type EndStatementContext<'input> = BaseParserRuleContext<'input,EndStatementContextExt<'input>>;

#[derive(Clone)]
pub struct EndStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for EndStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for EndStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_endStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_endStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for EndStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_endStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EndStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_endStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_endStatement }
}
antlr_rust::tid!{EndStatementContextExt<'a>}

impl<'input> EndStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EndStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EndStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EndStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<EndStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> EndStatementContextAttrs<'input> for EndStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn endStatement(&mut self,)
	-> Result<Rc<EndStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EndStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_endStatement);
        let mut _localctx: Rc<EndStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(216);
			recog.base.match_token(END,&mut recog.err_handler)?;

			recog.base.set_state(217);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forStatement ----------------
pub type ForStatementContextAll<'input> = ForStatementContext<'input>;


pub type ForStatementContext<'input> = BaseParserRuleContext<'input,ForStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ForStatementContextExt<'input>{
	pub body: Option<Rc<StatementOrScopeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ForStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ForStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_forStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_forStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ForStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_forStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forStatement }
}
antlr_rust::tid!{ForStatementContextExt<'a>}

impl<'input> ForStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForStatementContextExt{
				body: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ForStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ForStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FOR
/// Returns `None` if there is no child corresponding to token FOR
fn FOR(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(FOR, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
fn statementOrScope(&self) -> Option<Rc<StatementOrScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn setExpression(&self) -> Option<Rc<SetExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn rangeExpression(&self) -> Option<Rc<RangeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}

}

impl<'input> ForStatementContextAttrs<'input> for ForStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forStatement(&mut self,)
	-> Result<Rc<ForStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_forStatement);
        let mut _localctx: Rc<ForStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(219);
			recog.base.match_token(FOR,&mut recog.err_handler)?;

			/*InvokeRule scalarType*/
			recog.base.set_state(220);
			recog.scalarType()?;

			recog.base.set_state(221);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(222);
			recog.base.match_token(IN,&mut recog.err_handler)?;

			recog.base.set_state(229);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LBRACE 
				=> {
					{
					/*InvokeRule setExpression*/
					recog.base.set_state(223);
					recog.setExpression()?;

					}
				}

			 LBRACKET 
				=> {
					{
					recog.base.set_state(224);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					/*InvokeRule rangeExpression*/
					recog.base.set_state(225);
					recog.rangeExpression()?;

					recog.base.set_state(226);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}

			 Identifier 
				=> {
					{
					recog.base.set_state(228);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule statementOrScope*/
			recog.base.set_state(231);
			let tmp = recog.statementOrScope()?;
			 cast_mut::<_,ForStatementContext >(&mut _localctx).body = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ifStatement ----------------
pub type IfStatementContextAll<'input> = IfStatementContext<'input>;


pub type IfStatementContext<'input> = BaseParserRuleContext<'input,IfStatementContextExt<'input>>;

#[derive(Clone)]
pub struct IfStatementContextExt<'input>{
	pub if_body: Option<Rc<StatementOrScopeContextAll<'input>>>,
	pub else_body: Option<Rc<StatementOrScopeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for IfStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for IfStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ifStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_ifStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for IfStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_ifStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ifStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ifStatement }
}
antlr_rust::tid!{IfStatementContextExt<'a>}

impl<'input> IfStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IfStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IfStatementContextExt{
				if_body: None, else_body: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait IfStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<IfStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn statementOrScope_all(&self) ->  Vec<Rc<StatementOrScopeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statementOrScope(&self, i: usize) -> Option<Rc<StatementOrScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}

}

impl<'input> IfStatementContextAttrs<'input> for IfStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ifStatement(&mut self,)
	-> Result<Rc<IfStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IfStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_ifStatement);
        let mut _localctx: Rc<IfStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(233);
			recog.base.match_token(IF,&mut recog.err_handler)?;

			recog.base.set_state(234);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(235);
			recog.expression_rec(0)?;

			recog.base.set_state(236);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			/*InvokeRule statementOrScope*/
			recog.base.set_state(237);
			let tmp = recog.statementOrScope()?;
			 cast_mut::<_,IfStatementContext >(&mut _localctx).if_body = Some(tmp.clone());
			  

			recog.base.set_state(240);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(238);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					/*InvokeRule statementOrScope*/
					recog.base.set_state(239);
					let tmp = recog.statementOrScope()?;
					 cast_mut::<_,IfStatementContext >(&mut _localctx).else_body = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- returnStatement ----------------
pub type ReturnStatementContextAll<'input> = ReturnStatementContext<'input>;


pub type ReturnStatementContext<'input> = BaseParserRuleContext<'input,ReturnStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ReturnStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ReturnStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_returnStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ReturnStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_returnStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnStatement }
}
antlr_rust::tid!{ReturnStatementContextExt<'a>}

impl<'input> ReturnStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ReturnStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn measureExpression(&self) -> Option<Rc<MeasureExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnStatementContextAttrs<'input> for ReturnStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnStatement(&mut self,)
	-> Result<Rc<ReturnStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_returnStatement);
        let mut _localctx: Rc<ReturnStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(242);
			recog.base.match_token(RETURN,&mut recog.err_handler)?;

			recog.base.set_state(245);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | ARRAY | DURATION |
			 STRETCH | DURATIONOF | BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT |
			 ImaginaryLiteral | BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
			 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
			 BitstringLiteral 
				=> {
			    	{
			    	/*InvokeRule expression*/
			    	recog.base.set_state(243);
			    	recog.expression_rec(0)?;

			    	}
			    }

			 MEASURE 
				=> {
			    	{
			    	/*InvokeRule measureExpression*/
			    	recog.base.set_state(244);
			    	recog.measureExpression()?;

			    	}
			    }

			 SEMICOLON 
				=> {
			    }

				_ => {}
			}
			recog.base.set_state(247);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- whileStatement ----------------
pub type WhileStatementContextAll<'input> = WhileStatementContext<'input>;


pub type WhileStatementContext<'input> = BaseParserRuleContext<'input,WhileStatementContextExt<'input>>;

#[derive(Clone)]
pub struct WhileStatementContextExt<'input>{
	pub body: Option<Rc<StatementOrScopeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for WhileStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for WhileStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_whileStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_whileStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for WhileStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_whileStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhileStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whileStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whileStatement }
}
antlr_rust::tid!{WhileStatementContextExt<'a>}

impl<'input> WhileStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhileStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhileStatementContextExt{
				body: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait WhileStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<WhileStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WHILE
/// Returns `None` if there is no child corresponding to token WHILE
fn WHILE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(WHILE, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn statementOrScope(&self) -> Option<Rc<StatementOrScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WhileStatementContextAttrs<'input> for WhileStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn whileStatement(&mut self,)
	-> Result<Rc<WhileStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WhileStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_whileStatement);
        let mut _localctx: Rc<WhileStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(249);
			recog.base.match_token(WHILE,&mut recog.err_handler)?;

			recog.base.set_state(250);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(251);
			recog.expression_rec(0)?;

			recog.base.set_state(252);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			/*InvokeRule statementOrScope*/
			recog.base.set_state(253);
			let tmp = recog.statementOrScope()?;
			 cast_mut::<_,WhileStatementContext >(&mut _localctx).body = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- barrierStatement ----------------
pub type BarrierStatementContextAll<'input> = BarrierStatementContext<'input>;


pub type BarrierStatementContext<'input> = BaseParserRuleContext<'input,BarrierStatementContextExt<'input>>;

#[derive(Clone)]
pub struct BarrierStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for BarrierStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for BarrierStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_barrierStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_barrierStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for BarrierStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_barrierStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for BarrierStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_barrierStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_barrierStatement }
}
antlr_rust::tid!{BarrierStatementContextExt<'a>}

impl<'input> BarrierStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BarrierStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BarrierStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BarrierStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<BarrierStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BARRIER
/// Returns `None` if there is no child corresponding to token BARRIER
fn BARRIER(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(BARRIER, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
fn gateOperandList(&self) -> Option<Rc<GateOperandListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BarrierStatementContextAttrs<'input> for BarrierStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn barrierStatement(&mut self,)
	-> Result<Rc<BarrierStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BarrierStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_barrierStatement);
        let mut _localctx: Rc<BarrierStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(255);
			recog.base.match_token(BARRIER,&mut recog.err_handler)?;

			recog.base.set_state(257);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Identifier || _la==HardwareQubit {
				{
				/*InvokeRule gateOperandList*/
				recog.base.set_state(256);
				recog.gateOperandList()?;

				}
			}

			recog.base.set_state(259);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- boxStatement ----------------
pub type BoxStatementContextAll<'input> = BoxStatementContext<'input>;


pub type BoxStatementContext<'input> = BaseParserRuleContext<'input,BoxStatementContextExt<'input>>;

#[derive(Clone)]
pub struct BoxStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for BoxStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for BoxStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_boxStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_boxStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for BoxStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_boxStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for BoxStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_boxStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_boxStatement }
}
antlr_rust::tid!{BoxStatementContextExt<'a>}

impl<'input> BoxStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BoxStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BoxStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BoxStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<BoxStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BOX
/// Returns `None` if there is no child corresponding to token BOX
fn BOX(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(BOX, 0)
}
fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BoxStatementContextAttrs<'input> for BoxStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn boxStatement(&mut self,)
	-> Result<Rc<BoxStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BoxStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_boxStatement);
        let mut _localctx: Rc<BoxStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(261);
			recog.base.match_token(BOX,&mut recog.err_handler)?;

			recog.base.set_state(263);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACKET {
				{
				/*InvokeRule designator*/
				recog.base.set_state(262);
				recog.designator()?;

				}
			}

			/*InvokeRule scope*/
			recog.base.set_state(265);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- delayStatement ----------------
pub type DelayStatementContextAll<'input> = DelayStatementContext<'input>;


pub type DelayStatementContext<'input> = BaseParserRuleContext<'input,DelayStatementContextExt<'input>>;

#[derive(Clone)]
pub struct DelayStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DelayStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DelayStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_delayStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_delayStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DelayStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_delayStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for DelayStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_delayStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_delayStatement }
}
antlr_rust::tid!{DelayStatementContextExt<'a>}

impl<'input> DelayStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DelayStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DelayStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DelayStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DelayStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DELAY
/// Returns `None` if there is no child corresponding to token DELAY
fn DELAY(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DELAY, 0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
fn gateOperandList(&self) -> Option<Rc<GateOperandListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DelayStatementContextAttrs<'input> for DelayStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn delayStatement(&mut self,)
	-> Result<Rc<DelayStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DelayStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_delayStatement);
        let mut _localctx: Rc<DelayStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(267);
			recog.base.match_token(DELAY,&mut recog.err_handler)?;

			/*InvokeRule designator*/
			recog.base.set_state(268);
			recog.designator()?;

			recog.base.set_state(270);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Identifier || _la==HardwareQubit {
				{
				/*InvokeRule gateOperandList*/
				recog.base.set_state(269);
				recog.gateOperandList()?;

				}
			}

			recog.base.set_state(272);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- gateCallStatement ----------------
pub type GateCallStatementContextAll<'input> = GateCallStatementContext<'input>;


pub type GateCallStatementContext<'input> = BaseParserRuleContext<'input,GateCallStatementContextExt<'input>>;

#[derive(Clone)]
pub struct GateCallStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for GateCallStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for GateCallStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_gateCallStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_gateCallStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for GateCallStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_gateCallStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for GateCallStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gateCallStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gateCallStatement }
}
antlr_rust::tid!{GateCallStatementContextExt<'a>}

impl<'input> GateCallStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GateCallStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GateCallStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GateCallStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<GateCallStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn gateOperandList(&self) -> Option<Rc<GateOperandListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
fn gateModifier_all(&self) ->  Vec<Rc<GateModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn gateModifier(&self, i: usize) -> Option<Rc<GateModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token GPHASE
/// Returns `None` if there is no child corresponding to token GPHASE
fn GPHASE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(GPHASE, 0)
}

}

impl<'input> GateCallStatementContextAttrs<'input> for GateCallStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn gateCallStatement(&mut self,)
	-> Result<Rc<GateCallStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GateCallStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_gateCallStatement);
        let mut _localctx: Rc<GateCallStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(315);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(277);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 42)) & !0x3f) == 0 && ((1usize << (_la - 42)) & ((1usize << (INV - 42)) | (1usize << (POW - 42)) | (1usize << (CTRL - 42)) | (1usize << (NEGCTRL - 42)))) != 0) {
						{
						{
						/*InvokeRule gateModifier*/
						recog.base.set_state(274);
						recog.gateModifier()?;

						}
						}
						recog.base.set_state(279);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(280);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(286);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						recog.base.set_state(281);
						recog.base.match_token(LPAREN,&mut recog.err_handler)?;

						recog.base.set_state(283);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if ((((_la - 30)) & !0x3f) == 0 && ((1usize << (_la - 30)) & ((1usize << (BOOL - 30)) | (1usize << (BIT - 30)) | (1usize << (INT - 30)) | (1usize << (UINT - 30)) | (1usize << (FLOAT - 30)) | (1usize << (ANGLE - 30)) | (1usize << (COMPLEX - 30)) | (1usize << (ARRAY - 30)) | (1usize << (DURATION - 30)) | (1usize << (STRETCH - 30)) | (1usize << (DURATIONOF - 30)) | (1usize << (BooleanLiteral - 30)) | (1usize << (LPAREN - 30)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
							{
							/*InvokeRule expressionList*/
							recog.base.set_state(282);
							recog.expressionList()?;

							}
						}

						recog.base.set_state(285);
						recog.base.match_token(RPAREN,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(289);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(288);
						recog.designator()?;

						}
					}

					/*InvokeRule gateOperandList*/
					recog.base.set_state(291);
					recog.gateOperandList()?;

					recog.base.set_state(292);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(297);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 42)) & !0x3f) == 0 && ((1usize << (_la - 42)) & ((1usize << (INV - 42)) | (1usize << (POW - 42)) | (1usize << (CTRL - 42)) | (1usize << (NEGCTRL - 42)))) != 0) {
						{
						{
						/*InvokeRule gateModifier*/
						recog.base.set_state(294);
						recog.gateModifier()?;

						}
						}
						recog.base.set_state(299);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(300);
					recog.base.match_token(GPHASE,&mut recog.err_handler)?;

					recog.base.set_state(306);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						recog.base.set_state(301);
						recog.base.match_token(LPAREN,&mut recog.err_handler)?;

						recog.base.set_state(303);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if ((((_la - 30)) & !0x3f) == 0 && ((1usize << (_la - 30)) & ((1usize << (BOOL - 30)) | (1usize << (BIT - 30)) | (1usize << (INT - 30)) | (1usize << (UINT - 30)) | (1usize << (FLOAT - 30)) | (1usize << (ANGLE - 30)) | (1usize << (COMPLEX - 30)) | (1usize << (ARRAY - 30)) | (1usize << (DURATION - 30)) | (1usize << (STRETCH - 30)) | (1usize << (DURATIONOF - 30)) | (1usize << (BooleanLiteral - 30)) | (1usize << (LPAREN - 30)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
							{
							/*InvokeRule expressionList*/
							recog.base.set_state(302);
							recog.expressionList()?;

							}
						}

						recog.base.set_state(305);
						recog.base.match_token(RPAREN,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(309);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(308);
						recog.designator()?;

						}
					}

					recog.base.set_state(312);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Identifier || _la==HardwareQubit {
						{
						/*InvokeRule gateOperandList*/
						recog.base.set_state(311);
						recog.gateOperandList()?;

						}
					}

					recog.base.set_state(314);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- measureArrowAssignmentStatement ----------------
pub type MeasureArrowAssignmentStatementContextAll<'input> = MeasureArrowAssignmentStatementContext<'input>;


pub type MeasureArrowAssignmentStatementContext<'input> = BaseParserRuleContext<'input,MeasureArrowAssignmentStatementContextExt<'input>>;

#[derive(Clone)]
pub struct MeasureArrowAssignmentStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for MeasureArrowAssignmentStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for MeasureArrowAssignmentStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_measureArrowAssignmentStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_measureArrowAssignmentStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for MeasureArrowAssignmentStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_measureArrowAssignmentStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for MeasureArrowAssignmentStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_measureArrowAssignmentStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_measureArrowAssignmentStatement }
}
antlr_rust::tid!{MeasureArrowAssignmentStatementContextExt<'a>}

impl<'input> MeasureArrowAssignmentStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MeasureArrowAssignmentStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MeasureArrowAssignmentStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MeasureArrowAssignmentStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<MeasureArrowAssignmentStatementContextExt<'input>>{

fn measureExpression(&self) -> Option<Rc<MeasureExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn indexedIdentifier(&self) -> Option<Rc<IndexedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MeasureArrowAssignmentStatementContextAttrs<'input> for MeasureArrowAssignmentStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn measureArrowAssignmentStatement(&mut self,)
	-> Result<Rc<MeasureArrowAssignmentStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MeasureArrowAssignmentStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_measureArrowAssignmentStatement);
        let mut _localctx: Rc<MeasureArrowAssignmentStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule measureExpression*/
			recog.base.set_state(317);
			recog.measureExpression()?;

			recog.base.set_state(320);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ARROW {
				{
				recog.base.set_state(318);
				recog.base.match_token(ARROW,&mut recog.err_handler)?;

				/*InvokeRule indexedIdentifier*/
				recog.base.set_state(319);
				recog.indexedIdentifier()?;

				}
			}

			recog.base.set_state(322);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- resetStatement ----------------
pub type ResetStatementContextAll<'input> = ResetStatementContext<'input>;


pub type ResetStatementContext<'input> = BaseParserRuleContext<'input,ResetStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ResetStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ResetStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ResetStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_resetStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_resetStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ResetStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_resetStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ResetStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_resetStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_resetStatement }
}
antlr_rust::tid!{ResetStatementContextExt<'a>}

impl<'input> ResetStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ResetStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ResetStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ResetStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ResetStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RESET
/// Returns `None` if there is no child corresponding to token RESET
fn RESET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RESET, 0)
}
fn gateOperand(&self) -> Option<Rc<GateOperandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> ResetStatementContextAttrs<'input> for ResetStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn resetStatement(&mut self,)
	-> Result<Rc<ResetStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ResetStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_resetStatement);
        let mut _localctx: Rc<ResetStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(324);
			recog.base.match_token(RESET,&mut recog.err_handler)?;

			/*InvokeRule gateOperand*/
			recog.base.set_state(325);
			recog.gateOperand()?;

			recog.base.set_state(326);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- aliasDeclarationStatement ----------------
pub type AliasDeclarationStatementContextAll<'input> = AliasDeclarationStatementContext<'input>;


pub type AliasDeclarationStatementContext<'input> = BaseParserRuleContext<'input,AliasDeclarationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct AliasDeclarationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for AliasDeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for AliasDeclarationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_aliasDeclarationStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_aliasDeclarationStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for AliasDeclarationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_aliasDeclarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for AliasDeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aliasDeclarationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aliasDeclarationStatement }
}
antlr_rust::tid!{AliasDeclarationStatementContextExt<'a>}

impl<'input> AliasDeclarationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AliasDeclarationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AliasDeclarationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AliasDeclarationStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<AliasDeclarationStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LET
/// Returns `None` if there is no child corresponding to token LET
fn LET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LET, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn aliasExpression(&self) -> Option<Rc<AliasExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> AliasDeclarationStatementContextAttrs<'input> for AliasDeclarationStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn aliasDeclarationStatement(&mut self,)
	-> Result<Rc<AliasDeclarationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AliasDeclarationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_aliasDeclarationStatement);
        let mut _localctx: Rc<AliasDeclarationStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(328);
			recog.base.match_token(LET,&mut recog.err_handler)?;

			recog.base.set_state(329);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(330);
			recog.base.match_token(EQUALS,&mut recog.err_handler)?;

			/*InvokeRule aliasExpression*/
			recog.base.set_state(331);
			recog.aliasExpression()?;

			recog.base.set_state(332);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- classicalDeclarationStatement ----------------
pub type ClassicalDeclarationStatementContextAll<'input> = ClassicalDeclarationStatementContext<'input>;


pub type ClassicalDeclarationStatementContext<'input> = BaseParserRuleContext<'input,ClassicalDeclarationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ClassicalDeclarationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ClassicalDeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ClassicalDeclarationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_classicalDeclarationStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_classicalDeclarationStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ClassicalDeclarationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_classicalDeclarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ClassicalDeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_classicalDeclarationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_classicalDeclarationStatement }
}
antlr_rust::tid!{ClassicalDeclarationStatementContextExt<'a>}

impl<'input> ClassicalDeclarationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClassicalDeclarationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClassicalDeclarationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClassicalDeclarationStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ClassicalDeclarationStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arrayType(&self) -> Option<Rc<ArrayTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn declarationExpression(&self) -> Option<Rc<DeclarationExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ClassicalDeclarationStatementContextAttrs<'input> for ClassicalDeclarationStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn classicalDeclarationStatement(&mut self,)
	-> Result<Rc<ClassicalDeclarationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClassicalDeclarationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_classicalDeclarationStatement);
        let mut _localctx: Rc<ClassicalDeclarationStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(336);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | DURATION | STRETCH 
				=> {
					{
					/*InvokeRule scalarType*/
					recog.base.set_state(334);
					recog.scalarType()?;

					}
				}

			 ARRAY 
				=> {
					{
					/*InvokeRule arrayType*/
					recog.base.set_state(335);
					recog.arrayType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(338);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(341);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EQUALS {
				{
				recog.base.set_state(339);
				recog.base.match_token(EQUALS,&mut recog.err_handler)?;

				/*InvokeRule declarationExpression*/
				recog.base.set_state(340);
				recog.declarationExpression()?;

				}
			}

			recog.base.set_state(343);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constDeclarationStatement ----------------
pub type ConstDeclarationStatementContextAll<'input> = ConstDeclarationStatementContext<'input>;


pub type ConstDeclarationStatementContext<'input> = BaseParserRuleContext<'input,ConstDeclarationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ConstDeclarationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ConstDeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ConstDeclarationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constDeclarationStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_constDeclarationStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ConstDeclarationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_constDeclarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstDeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constDeclarationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constDeclarationStatement }
}
antlr_rust::tid!{ConstDeclarationStatementContextExt<'a>}

impl<'input> ConstDeclarationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstDeclarationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstDeclarationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstDeclarationStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ConstDeclarationStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONST
/// Returns `None` if there is no child corresponding to token CONST
fn CONST(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CONST, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn declarationExpression(&self) -> Option<Rc<DeclarationExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> ConstDeclarationStatementContextAttrs<'input> for ConstDeclarationStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constDeclarationStatement(&mut self,)
	-> Result<Rc<ConstDeclarationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstDeclarationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_constDeclarationStatement);
        let mut _localctx: Rc<ConstDeclarationStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(345);
			recog.base.match_token(CONST,&mut recog.err_handler)?;

			/*InvokeRule scalarType*/
			recog.base.set_state(346);
			recog.scalarType()?;

			recog.base.set_state(347);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(348);
			recog.base.match_token(EQUALS,&mut recog.err_handler)?;

			/*InvokeRule declarationExpression*/
			recog.base.set_state(349);
			recog.declarationExpression()?;

			recog.base.set_state(350);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ioDeclarationStatement ----------------
pub type IoDeclarationStatementContextAll<'input> = IoDeclarationStatementContext<'input>;


pub type IoDeclarationStatementContext<'input> = BaseParserRuleContext<'input,IoDeclarationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct IoDeclarationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for IoDeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for IoDeclarationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ioDeclarationStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_ioDeclarationStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for IoDeclarationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_ioDeclarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for IoDeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ioDeclarationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ioDeclarationStatement }
}
antlr_rust::tid!{IoDeclarationStatementContextExt<'a>}

impl<'input> IoDeclarationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IoDeclarationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IoDeclarationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IoDeclarationStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<IoDeclarationStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token INPUT
/// Returns `None` if there is no child corresponding to token INPUT
fn INPUT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(INPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token OUTPUT
/// Returns `None` if there is no child corresponding to token OUTPUT
fn OUTPUT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(OUTPUT, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arrayType(&self) -> Option<Rc<ArrayTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IoDeclarationStatementContextAttrs<'input> for IoDeclarationStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ioDeclarationStatement(&mut self,)
	-> Result<Rc<IoDeclarationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IoDeclarationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_ioDeclarationStatement);
        let mut _localctx: Rc<IoDeclarationStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(352);
			_la = recog.base.input.la(1);
			if { !(_la==INPUT || _la==OUTPUT) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(355);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | DURATION | STRETCH 
				=> {
					{
					/*InvokeRule scalarType*/
					recog.base.set_state(353);
					recog.scalarType()?;

					}
				}

			 ARRAY 
				=> {
					{
					/*InvokeRule arrayType*/
					recog.base.set_state(354);
					recog.arrayType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(357);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(358);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- oldStyleDeclarationStatement ----------------
pub type OldStyleDeclarationStatementContextAll<'input> = OldStyleDeclarationStatementContext<'input>;


pub type OldStyleDeclarationStatementContext<'input> = BaseParserRuleContext<'input,OldStyleDeclarationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct OldStyleDeclarationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for OldStyleDeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for OldStyleDeclarationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_oldStyleDeclarationStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_oldStyleDeclarationStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for OldStyleDeclarationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_oldStyleDeclarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for OldStyleDeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oldStyleDeclarationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oldStyleDeclarationStatement }
}
antlr_rust::tid!{OldStyleDeclarationStatementContextExt<'a>}

impl<'input> OldStyleDeclarationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OldStyleDeclarationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OldStyleDeclarationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OldStyleDeclarationStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<OldStyleDeclarationStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token CREG
/// Returns `None` if there is no child corresponding to token CREG
fn CREG(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CREG, 0)
}
/// Retrieves first TerminalNode corresponding to token QREG
/// Returns `None` if there is no child corresponding to token QREG
fn QREG(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(QREG, 0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OldStyleDeclarationStatementContextAttrs<'input> for OldStyleDeclarationStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn oldStyleDeclarationStatement(&mut self,)
	-> Result<Rc<OldStyleDeclarationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OldStyleDeclarationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_oldStyleDeclarationStatement);
        let mut _localctx: Rc<OldStyleDeclarationStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(360);
			_la = recog.base.input.la(1);
			if { !(_la==QREG || _la==CREG) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(361);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(363);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACKET {
				{
				/*InvokeRule designator*/
				recog.base.set_state(362);
				recog.designator()?;

				}
			}

			recog.base.set_state(365);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- quantumDeclarationStatement ----------------
pub type QuantumDeclarationStatementContextAll<'input> = QuantumDeclarationStatementContext<'input>;


pub type QuantumDeclarationStatementContext<'input> = BaseParserRuleContext<'input,QuantumDeclarationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct QuantumDeclarationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for QuantumDeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for QuantumDeclarationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_quantumDeclarationStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_quantumDeclarationStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for QuantumDeclarationStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_quantumDeclarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for QuantumDeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_quantumDeclarationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_quantumDeclarationStatement }
}
antlr_rust::tid!{QuantumDeclarationStatementContextExt<'a>}

impl<'input> QuantumDeclarationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QuantumDeclarationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QuantumDeclarationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QuantumDeclarationStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<QuantumDeclarationStatementContextExt<'input>>{

fn qubitType(&self) -> Option<Rc<QubitTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> QuantumDeclarationStatementContextAttrs<'input> for QuantumDeclarationStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn quantumDeclarationStatement(&mut self,)
	-> Result<Rc<QuantumDeclarationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QuantumDeclarationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_quantumDeclarationStatement);
        let mut _localctx: Rc<QuantumDeclarationStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule qubitType*/
			recog.base.set_state(367);
			recog.qubitType()?;

			recog.base.set_state(368);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(369);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defStatement ----------------
pub type DefStatementContextAll<'input> = DefStatementContext<'input>;


pub type DefStatementContext<'input> = BaseParserRuleContext<'input,DefStatementContextExt<'input>>;

#[derive(Clone)]
pub struct DefStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DefStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DefStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_defStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DefStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_defStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defStatement }
}
antlr_rust::tid!{DefStatementContextExt<'a>}

impl<'input> DefStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DefStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEF
/// Returns `None` if there is no child corresponding to token DEF
fn DEF(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DEF, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn argumentDefinitionList(&self) -> Option<Rc<ArgumentDefinitionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnSignature(&self) -> Option<Rc<ReturnSignatureContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DefStatementContextAttrs<'input> for DefStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defStatement(&mut self,)
	-> Result<Rc<DefStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_defStatement);
        let mut _localctx: Rc<DefStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(371);
			recog.base.match_token(DEF,&mut recog.err_handler)?;

			recog.base.set_state(372);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(373);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(375);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 25)) & !0x3f) == 0 && ((1usize << (_la - 25)) & ((1usize << (READONLY - 25)) | (1usize << (MUTABLE - 25)) | (1usize << (QREG - 25)) | (1usize << (QUBIT - 25)) | (1usize << (CREG - 25)) | (1usize << (BOOL - 25)) | (1usize << (BIT - 25)) | (1usize << (INT - 25)) | (1usize << (UINT - 25)) | (1usize << (FLOAT - 25)) | (1usize << (ANGLE - 25)) | (1usize << (COMPLEX - 25)) | (1usize << (DURATION - 25)) | (1usize << (STRETCH - 25)))) != 0) {
				{
				/*InvokeRule argumentDefinitionList*/
				recog.base.set_state(374);
				recog.argumentDefinitionList()?;

				}
			}

			recog.base.set_state(377);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(379);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ARROW {
				{
				/*InvokeRule returnSignature*/
				recog.base.set_state(378);
				recog.returnSignature()?;

				}
			}

			/*InvokeRule scope*/
			recog.base.set_state(381);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- externStatement ----------------
pub type ExternStatementContextAll<'input> = ExternStatementContext<'input>;


pub type ExternStatementContext<'input> = BaseParserRuleContext<'input,ExternStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExternStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ExternStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ExternStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_externStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_externStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ExternStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_externStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExternStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externStatement }
}
antlr_rust::tid!{ExternStatementContextExt<'a>}

impl<'input> ExternStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExternStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExternStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExternStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ExternStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EXTERN
/// Returns `None` if there is no child corresponding to token EXTERN
fn EXTERN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(EXTERN, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
fn externArgumentList(&self) -> Option<Rc<ExternArgumentListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnSignature(&self) -> Option<Rc<ReturnSignatureContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExternStatementContextAttrs<'input> for ExternStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn externStatement(&mut self,)
	-> Result<Rc<ExternStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExternStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_externStatement);
        let mut _localctx: Rc<ExternStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(383);
			recog.base.match_token(EXTERN,&mut recog.err_handler)?;

			recog.base.set_state(384);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(385);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(387);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 25)) & !0x3f) == 0 && ((1usize << (_la - 25)) & ((1usize << (READONLY - 25)) | (1usize << (MUTABLE - 25)) | (1usize << (CREG - 25)) | (1usize << (BOOL - 25)) | (1usize << (BIT - 25)) | (1usize << (INT - 25)) | (1usize << (UINT - 25)) | (1usize << (FLOAT - 25)) | (1usize << (ANGLE - 25)) | (1usize << (COMPLEX - 25)) | (1usize << (DURATION - 25)) | (1usize << (STRETCH - 25)))) != 0) {
				{
				/*InvokeRule externArgumentList*/
				recog.base.set_state(386);
				recog.externArgumentList()?;

				}
			}

			recog.base.set_state(389);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(391);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ARROW {
				{
				/*InvokeRule returnSignature*/
				recog.base.set_state(390);
				recog.returnSignature()?;

				}
			}

			recog.base.set_state(393);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- gateStatement ----------------
pub type GateStatementContextAll<'input> = GateStatementContext<'input>;


pub type GateStatementContext<'input> = BaseParserRuleContext<'input,GateStatementContextExt<'input>>;

#[derive(Clone)]
pub struct GateStatementContextExt<'input>{
	pub params: Option<Rc<IdentifierListContextAll<'input>>>,
	pub qubits: Option<Rc<IdentifierListContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for GateStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for GateStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_gateStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_gateStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for GateStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_gateStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for GateStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gateStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gateStatement }
}
antlr_rust::tid!{GateStatementContextExt<'a>}

impl<'input> GateStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GateStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GateStatementContextExt{
				params: None, qubits: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait GateStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<GateStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GATE
/// Returns `None` if there is no child corresponding to token GATE
fn GATE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(GATE, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifierList_all(&self) ->  Vec<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifierList(&self, i: usize) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> GateStatementContextAttrs<'input> for GateStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn gateStatement(&mut self,)
	-> Result<Rc<GateStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GateStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_gateStatement);
        let mut _localctx: Rc<GateStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(395);
			recog.base.match_token(GATE,&mut recog.err_handler)?;

			recog.base.set_state(396);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(402);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LPAREN {
				{
				recog.base.set_state(397);
				recog.base.match_token(LPAREN,&mut recog.err_handler)?;

				recog.base.set_state(399);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==Identifier {
					{
					/*InvokeRule identifierList*/
					recog.base.set_state(398);
					let tmp = recog.identifierList()?;
					 cast_mut::<_,GateStatementContext >(&mut _localctx).params = Some(tmp.clone());
					  

					}
				}

				recog.base.set_state(401);
				recog.base.match_token(RPAREN,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule identifierList*/
			recog.base.set_state(404);
			let tmp = recog.identifierList()?;
			 cast_mut::<_,GateStatementContext >(&mut _localctx).qubits = Some(tmp.clone());
			  

			/*InvokeRule scope*/
			recog.base.set_state(405);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assignmentStatement ----------------
pub type AssignmentStatementContextAll<'input> = AssignmentStatementContext<'input>;


pub type AssignmentStatementContext<'input> = BaseParserRuleContext<'input,AssignmentStatementContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentStatementContextExt<'input>{
	pub op: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for AssignmentStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for AssignmentStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assignmentStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_assignmentStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for AssignmentStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_assignmentStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignmentStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignmentStatement }
}
antlr_rust::tid!{AssignmentStatementContextExt<'a>}

impl<'input> AssignmentStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentStatementContextExt{
				op: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<AssignmentStatementContextExt<'input>>{

fn indexedIdentifier(&self) -> Option<Rc<IndexedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
/// Retrieves first TerminalNode corresponding to token CompoundAssignmentOperator
/// Returns `None` if there is no child corresponding to token CompoundAssignmentOperator
fn CompoundAssignmentOperator(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CompoundAssignmentOperator, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn measureExpression(&self) -> Option<Rc<MeasureExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssignmentStatementContextAttrs<'input> for AssignmentStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignmentStatement(&mut self,)
	-> Result<Rc<AssignmentStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_assignmentStatement);
        let mut _localctx: Rc<AssignmentStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule indexedIdentifier*/
			recog.base.set_state(407);
			recog.indexedIdentifier()?;

			recog.base.set_state(408);
			 cast_mut::<_,AssignmentStatementContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
			 
			_la = recog.base.input.la(1);
			if { !(_la==EQUALS || _la==CompoundAssignmentOperator) } {
				let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
				 cast_mut::<_,AssignmentStatementContext >(&mut _localctx).op = Some(tmp.clone());
				  

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(411);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | ARRAY | DURATION |
			 STRETCH | DURATIONOF | BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT |
			 ImaginaryLiteral | BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
			 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
			 BitstringLiteral 
				=> {
					{
					/*InvokeRule expression*/
					recog.base.set_state(409);
					recog.expression_rec(0)?;

					}
				}

			 MEASURE 
				=> {
					{
					/*InvokeRule measureExpression*/
					recog.base.set_state(410);
					recog.measureExpression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(413);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expressionStatement ----------------
pub type ExpressionStatementContextAll<'input> = ExpressionStatementContext<'input>;


pub type ExpressionStatementContext<'input> = BaseParserRuleContext<'input,ExpressionStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ExpressionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ExpressionStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expressionStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_expressionStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ExpressionStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_expressionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionStatement }
}
antlr_rust::tid!{ExpressionStatementContextExt<'a>}

impl<'input> ExpressionStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ExpressionStatementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> ExpressionStatementContextAttrs<'input> for ExpressionStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionStatement(&mut self,)
	-> Result<Rc<ExpressionStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_expressionStatement);
        let mut _localctx: Rc<ExpressionStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(415);
			recog.expression_rec(0)?;

			recog.base.set_state(416);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- calStatement ----------------
pub type CalStatementContextAll<'input> = CalStatementContext<'input>;


pub type CalStatementContext<'input> = BaseParserRuleContext<'input,CalStatementContextExt<'input>>;

#[derive(Clone)]
pub struct CalStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for CalStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for CalStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_calStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_calStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for CalStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_calStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for CalStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_calStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_calStatement }
}
antlr_rust::tid!{CalStatementContextExt<'a>}

impl<'input> CalStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CalStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CalStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CalStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<CalStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CAL
/// Returns `None` if there is no child corresponding to token CAL
fn CAL(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CalibrationBlock
/// Returns `None` if there is no child corresponding to token CalibrationBlock
fn CalibrationBlock(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CalibrationBlock, 0)
}

}

impl<'input> CalStatementContextAttrs<'input> for CalStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn calStatement(&mut self,)
	-> Result<Rc<CalStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CalStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_calStatement);
        let mut _localctx: Rc<CalStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(418);
			recog.base.match_token(CAL,&mut recog.err_handler)?;

			recog.base.set_state(419);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(421);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==CalibrationBlock {
				{
				recog.base.set_state(420);
				recog.base.match_token(CalibrationBlock,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(423);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defcalStatement ----------------
pub type DefcalStatementContextAll<'input> = DefcalStatementContext<'input>;


pub type DefcalStatementContext<'input> = BaseParserRuleContext<'input,DefcalStatementContextExt<'input>>;

#[derive(Clone)]
pub struct DefcalStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DefcalStatementContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DefcalStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defcalStatement(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_defcalStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DefcalStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_defcalStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefcalStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defcalStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defcalStatement }
}
antlr_rust::tid!{DefcalStatementContextExt<'a>}

impl<'input> DefcalStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefcalStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefcalStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefcalStatementContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DefcalStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEFCAL
/// Returns `None` if there is no child corresponding to token DEFCAL
fn DEFCAL(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DEFCAL, 0)
}
fn defcalTarget(&self) -> Option<Rc<DefcalTargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defcalOperandList(&self) -> Option<Rc<DefcalOperandListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn returnSignature(&self) -> Option<Rc<ReturnSignatureContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CalibrationBlock
/// Returns `None` if there is no child corresponding to token CalibrationBlock
fn CalibrationBlock(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CalibrationBlock, 0)
}
fn defcalArgumentDefinitionList(&self) -> Option<Rc<DefcalArgumentDefinitionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DefcalStatementContextAttrs<'input> for DefcalStatementContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defcalStatement(&mut self,)
	-> Result<Rc<DefcalStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefcalStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_defcalStatement);
        let mut _localctx: Rc<DefcalStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(425);
			recog.base.match_token(DEFCAL,&mut recog.err_handler)?;

			/*InvokeRule defcalTarget*/
			recog.base.set_state(426);
			recog.defcalTarget()?;

			recog.base.set_state(432);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LPAREN {
				{
				recog.base.set_state(427);
				recog.base.match_token(LPAREN,&mut recog.err_handler)?;

				recog.base.set_state(429);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << READONLY) | (1usize << MUTABLE) | (1usize << QREG) | (1usize << QUBIT) | (1usize << CREG) | (1usize << BOOL) | (1usize << BIT))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (INT - 32)) | (1usize << (UINT - 32)) | (1usize << (FLOAT - 32)) | (1usize << (ANGLE - 32)) | (1usize << (COMPLEX - 32)) | (1usize << (ARRAY - 32)) | (1usize << (DURATION - 32)) | (1usize << (STRETCH - 32)) | (1usize << (DURATIONOF - 32)) | (1usize << (BooleanLiteral - 32)) | (1usize << (LPAREN - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
					{
					/*InvokeRule defcalArgumentDefinitionList*/
					recog.base.set_state(428);
					recog.defcalArgumentDefinitionList()?;

					}
				}

				recog.base.set_state(431);
				recog.base.match_token(RPAREN,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule defcalOperandList*/
			recog.base.set_state(434);
			recog.defcalOperandList()?;

			recog.base.set_state(436);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ARROW {
				{
				/*InvokeRule returnSignature*/
				recog.base.set_state(435);
				recog.returnSignature()?;

				}
			}

			recog.base.set_state(438);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(440);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==CalibrationBlock {
				{
				recog.base.set_state(439);
				recog.base.match_token(CalibrationBlock,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(442);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input>{
	BitwiseXorExpressionContext(BitwiseXorExpressionContext<'input>),
	AdditiveExpressionContext(AdditiveExpressionContext<'input>),
	DurationofExpressionContext(DurationofExpressionContext<'input>),
	ParenthesisExpressionContext(ParenthesisExpressionContext<'input>),
	ComparisonExpressionContext(ComparisonExpressionContext<'input>),
	MultiplicativeExpressionContext(MultiplicativeExpressionContext<'input>),
	LogicalOrExpressionContext(LogicalOrExpressionContext<'input>),
	CastExpressionContext(CastExpressionContext<'input>),
	PowerExpressionContext(PowerExpressionContext<'input>),
	BitwiseOrExpressionContext(BitwiseOrExpressionContext<'input>),
	CallExpressionContext(CallExpressionContext<'input>),
	BitshiftExpressionContext(BitshiftExpressionContext<'input>),
	BitwiseAndExpressionContext(BitwiseAndExpressionContext<'input>),
	EqualityExpressionContext(EqualityExpressionContext<'input>),
	LogicalAndExpressionContext(LogicalAndExpressionContext<'input>),
	IndexExpressionContext(IndexExpressionContext<'input>),
	UnaryExpressionContext(UnaryExpressionContext<'input>),
	LiteralExpressionContext(LiteralExpressionContext<'input>),
Error(ExpressionContext<'input>)
}
antlr_rust::tid!{ExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input>{}

impl<'input> qasm3ParserContext<'input> for ExpressionContextAll<'input>{}

impl<'input> Deref for ExpressionContextAll<'input>{
	type Target = dyn ExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExpressionContextAll::*;
		match self{
			BitwiseXorExpressionContext(inner) => inner,
			AdditiveExpressionContext(inner) => inner,
			DurationofExpressionContext(inner) => inner,
			ParenthesisExpressionContext(inner) => inner,
			ComparisonExpressionContext(inner) => inner,
			MultiplicativeExpressionContext(inner) => inner,
			LogicalOrExpressionContext(inner) => inner,
			CastExpressionContext(inner) => inner,
			PowerExpressionContext(inner) => inner,
			BitwiseOrExpressionContext(inner) => inner,
			CallExpressionContext(inner) => inner,
			BitshiftExpressionContext(inner) => inner,
			BitwiseAndExpressionContext(inner) => inner,
			EqualityExpressionContext(inner) => inner,
			LogicalAndExpressionContext(inner) => inner,
			IndexExpressionContext(inner) => inner,
			UnaryExpressionContext(inner) => inner,
			LiteralExpressionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn qasm3ParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn qasm3ParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
		ExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExpressionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{


}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

pub type BitwiseXorExpressionContext<'input> = BaseParserRuleContext<'input,BitwiseXorExpressionContextExt<'input>>;

pub trait BitwiseXorExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token CARET
	/// Returns `None` if there is no child corresponding to token CARET
	fn CARET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(CARET, 0)
	}
}

impl<'input> BitwiseXorExpressionContextAttrs<'input> for BitwiseXorExpressionContext<'input>{}

pub struct BitwiseXorExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BitwiseXorExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for BitwiseXorExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for BitwiseXorExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_bitwiseXorExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_bitwiseXorExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for BitwiseXorExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_bitwiseXorExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitwiseXorExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BitwiseXorExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BitwiseXorExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for BitwiseXorExpressionContext<'input> {}

impl<'input> BitwiseXorExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::BitwiseXorExpressionContext(
				BaseParserRuleContext::copy_from(ctx,BitwiseXorExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AdditiveExpressionContext<'input> = BaseParserRuleContext<'input,AdditiveExpressionContextExt<'input>>;

pub trait AdditiveExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token PLUS
	/// Returns `None` if there is no child corresponding to token PLUS
	fn PLUS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(PLUS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExpressionContext<'input>{}

pub struct AdditiveExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AdditiveExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for AdditiveExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for AdditiveExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_additiveExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_additiveExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for AdditiveExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_additiveExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditiveExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for AdditiveExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for AdditiveExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for AdditiveExpressionContext<'input> {}

impl<'input> AdditiveExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::AdditiveExpressionContext(
				BaseParserRuleContext::copy_from(ctx,AdditiveExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DurationofExpressionContext<'input> = BaseParserRuleContext<'input,DurationofExpressionContextExt<'input>>;

pub trait DurationofExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token DURATIONOF
	/// Returns `None` if there is no child corresponding to token DURATIONOF
	fn DURATIONOF(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(DURATIONOF, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
}

impl<'input> DurationofExpressionContextAttrs<'input> for DurationofExpressionContext<'input>{}

pub struct DurationofExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DurationofExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for DurationofExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DurationofExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_durationofExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_durationofExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DurationofExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_durationofExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for DurationofExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for DurationofExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for DurationofExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for DurationofExpressionContext<'input> {}

impl<'input> DurationofExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::DurationofExpressionContext(
				BaseParserRuleContext::copy_from(ctx,DurationofExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenthesisExpressionContext<'input> = BaseParserRuleContext<'input,ParenthesisExpressionContextExt<'input>>;

pub trait ParenthesisExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
}

impl<'input> ParenthesisExpressionContextAttrs<'input> for ParenthesisExpressionContext<'input>{}

pub struct ParenthesisExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParenthesisExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for ParenthesisExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ParenthesisExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parenthesisExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_parenthesisExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ParenthesisExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_parenthesisExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenthesisExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ParenthesisExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ParenthesisExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ParenthesisExpressionContext<'input> {}

impl<'input> ParenthesisExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ParenthesisExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ParenthesisExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ComparisonExpressionContext<'input> = BaseParserRuleContext<'input,ComparisonExpressionContextExt<'input>>;

pub trait ComparisonExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token ComparisonOperator
	/// Returns `None` if there is no child corresponding to token ComparisonOperator
	fn ComparisonOperator(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(ComparisonOperator, 0)
	}
}

impl<'input> ComparisonExpressionContextAttrs<'input> for ComparisonExpressionContext<'input>{}

pub struct ComparisonExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ComparisonExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for ComparisonExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ComparisonExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_comparisonExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_comparisonExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ComparisonExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_comparisonExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ComparisonExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ComparisonExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ComparisonExpressionContext<'input> {}

impl<'input> ComparisonExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ComparisonExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ComparisonExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultiplicativeExpressionContext<'input> = BaseParserRuleContext<'input,MultiplicativeExpressionContextExt<'input>>;

pub trait MultiplicativeExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token ASTERISK
	/// Returns `None` if there is no child corresponding to token ASTERISK
	fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(ASTERISK, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SLASH
	/// Returns `None` if there is no child corresponding to token SLASH
	fn SLASH(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(SLASH, 0)
	}
	/// Retrieves first TerminalNode corresponding to token PERCENT
	/// Returns `None` if there is no child corresponding to token PERCENT
	fn PERCENT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(PERCENT, 0)
	}
}

impl<'input> MultiplicativeExpressionContextAttrs<'input> for MultiplicativeExpressionContext<'input>{}

pub struct MultiplicativeExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MultiplicativeExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for MultiplicativeExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_multiplicativeExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_multiplicativeExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for MultiplicativeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_multiplicativeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for MultiplicativeExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for MultiplicativeExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for MultiplicativeExpressionContext<'input> {}

impl<'input> MultiplicativeExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::MultiplicativeExpressionContext(
				BaseParserRuleContext::copy_from(ctx,MultiplicativeExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicalOrExpressionContext<'input> = BaseParserRuleContext<'input,LogicalOrExpressionContextExt<'input>>;

pub trait LogicalOrExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token DOUBLE_PIPE
	/// Returns `None` if there is no child corresponding to token DOUBLE_PIPE
	fn DOUBLE_PIPE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(DOUBLE_PIPE, 0)
	}
}

impl<'input> LogicalOrExpressionContextAttrs<'input> for LogicalOrExpressionContext<'input>{}

pub struct LogicalOrExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicalOrExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for LogicalOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for LogicalOrExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_logicalOrExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_logicalOrExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for LogicalOrExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_logicalOrExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for LogicalOrExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for LogicalOrExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for LogicalOrExpressionContext<'input> {}

impl<'input> LogicalOrExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::LogicalOrExpressionContext(
				BaseParserRuleContext::copy_from(ctx,LogicalOrExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CastExpressionContext<'input> = BaseParserRuleContext<'input,CastExpressionContextExt<'input>>;

pub trait CastExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn arrayType(&self) -> Option<Rc<ArrayTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CastExpressionContextAttrs<'input> for CastExpressionContext<'input>{}

pub struct CastExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CastExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for CastExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for CastExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_castExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_castExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for CastExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_castExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for CastExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for CastExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for CastExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for CastExpressionContext<'input> {}

impl<'input> CastExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::CastExpressionContext(
				BaseParserRuleContext::copy_from(ctx,CastExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PowerExpressionContext<'input> = BaseParserRuleContext<'input,PowerExpressionContextExt<'input>>;

pub trait PowerExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token DOUBLE_ASTERISK
	/// Returns `None` if there is no child corresponding to token DOUBLE_ASTERISK
	fn DOUBLE_ASTERISK(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(DOUBLE_ASTERISK, 0)
	}
}

impl<'input> PowerExpressionContextAttrs<'input> for PowerExpressionContext<'input>{}

pub struct PowerExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PowerExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for PowerExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for PowerExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_powerExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_powerExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for PowerExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_powerExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowerExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for PowerExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for PowerExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for PowerExpressionContext<'input> {}

impl<'input> PowerExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::PowerExpressionContext(
				BaseParserRuleContext::copy_from(ctx,PowerExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BitwiseOrExpressionContext<'input> = BaseParserRuleContext<'input,BitwiseOrExpressionContextExt<'input>>;

pub trait BitwiseOrExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token PIPE
	/// Returns `None` if there is no child corresponding to token PIPE
	fn PIPE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(PIPE, 0)
	}
}

impl<'input> BitwiseOrExpressionContextAttrs<'input> for BitwiseOrExpressionContext<'input>{}

pub struct BitwiseOrExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BitwiseOrExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for BitwiseOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for BitwiseOrExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_bitwiseOrExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_bitwiseOrExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for BitwiseOrExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_bitwiseOrExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitwiseOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BitwiseOrExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BitwiseOrExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for BitwiseOrExpressionContext<'input> {}

impl<'input> BitwiseOrExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::BitwiseOrExpressionContext(
				BaseParserRuleContext::copy_from(ctx,BitwiseOrExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CallExpressionContext<'input> = BaseParserRuleContext<'input,CallExpressionContextExt<'input>>;

pub trait CallExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(Identifier, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CallExpressionContextAttrs<'input> for CallExpressionContext<'input>{}

pub struct CallExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CallExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for CallExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for CallExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_callExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_callExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for CallExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_callExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for CallExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for CallExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for CallExpressionContext<'input> {}

impl<'input> CallExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::CallExpressionContext(
				BaseParserRuleContext::copy_from(ctx,CallExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BitshiftExpressionContext<'input> = BaseParserRuleContext<'input,BitshiftExpressionContextExt<'input>>;

pub trait BitshiftExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token BitshiftOperator
	/// Returns `None` if there is no child corresponding to token BitshiftOperator
	fn BitshiftOperator(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(BitshiftOperator, 0)
	}
}

impl<'input> BitshiftExpressionContextAttrs<'input> for BitshiftExpressionContext<'input>{}

pub struct BitshiftExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BitshiftExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for BitshiftExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for BitshiftExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_bitshiftExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_bitshiftExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for BitshiftExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_bitshiftExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitshiftExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BitshiftExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BitshiftExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for BitshiftExpressionContext<'input> {}

impl<'input> BitshiftExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::BitshiftExpressionContext(
				BaseParserRuleContext::copy_from(ctx,BitshiftExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BitwiseAndExpressionContext<'input> = BaseParserRuleContext<'input,BitwiseAndExpressionContextExt<'input>>;

pub trait BitwiseAndExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token AMPERSAND
	/// Returns `None` if there is no child corresponding to token AMPERSAND
	fn AMPERSAND(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(AMPERSAND, 0)
	}
}

impl<'input> BitwiseAndExpressionContextAttrs<'input> for BitwiseAndExpressionContext<'input>{}

pub struct BitwiseAndExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BitwiseAndExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for BitwiseAndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for BitwiseAndExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_bitwiseAndExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_bitwiseAndExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for BitwiseAndExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_bitwiseAndExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitwiseAndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BitwiseAndExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BitwiseAndExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for BitwiseAndExpressionContext<'input> {}

impl<'input> BitwiseAndExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::BitwiseAndExpressionContext(
				BaseParserRuleContext::copy_from(ctx,BitwiseAndExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqualityExpressionContext<'input> = BaseParserRuleContext<'input,EqualityExpressionContextExt<'input>>;

pub trait EqualityExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token EqualityOperator
	/// Returns `None` if there is no child corresponding to token EqualityOperator
	fn EqualityOperator(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(EqualityOperator, 0)
	}
}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExpressionContext<'input>{}

pub struct EqualityExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EqualityExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for EqualityExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for EqualityExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_equalityExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_equalityExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for EqualityExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_equalityExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for EqualityExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for EqualityExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for EqualityExpressionContext<'input> {}

impl<'input> EqualityExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::EqualityExpressionContext(
				BaseParserRuleContext::copy_from(ctx,EqualityExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicalAndExpressionContext<'input> = BaseParserRuleContext<'input,LogicalAndExpressionContextExt<'input>>;

pub trait LogicalAndExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token DOUBLE_AMPERSAND
	/// Returns `None` if there is no child corresponding to token DOUBLE_AMPERSAND
	fn DOUBLE_AMPERSAND(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(DOUBLE_AMPERSAND, 0)
	}
}

impl<'input> LogicalAndExpressionContextAttrs<'input> for LogicalAndExpressionContext<'input>{}

pub struct LogicalAndExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicalAndExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for LogicalAndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for LogicalAndExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_logicalAndExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_logicalAndExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for LogicalAndExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_logicalAndExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalAndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for LogicalAndExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for LogicalAndExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for LogicalAndExpressionContext<'input> {}

impl<'input> LogicalAndExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::LogicalAndExpressionContext(
				BaseParserRuleContext::copy_from(ctx,LogicalAndExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IndexExpressionContext<'input> = BaseParserRuleContext<'input,IndexExpressionContextExt<'input>>;

pub trait IndexExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn indexOperator(&self) -> Option<Rc<IndexOperatorContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IndexExpressionContextAttrs<'input> for IndexExpressionContext<'input>{}

pub struct IndexExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IndexExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for IndexExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for IndexExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_indexExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_indexExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for IndexExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_indexExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for IndexExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for IndexExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for IndexExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for IndexExpressionContext<'input> {}

impl<'input> IndexExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::IndexExpressionContext(
				BaseParserRuleContext::copy_from(ctx,IndexExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryExpressionContext<'input> = BaseParserRuleContext<'input,UnaryExpressionContextExt<'input>>;

pub trait UnaryExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token TILDE
	/// Returns `None` if there is no child corresponding to token TILDE
	fn TILDE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(TILDE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token EXCLAMATION_POINT
	/// Returns `None` if there is no child corresponding to token EXCLAMATION_POINT
	fn EXCLAMATION_POINT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(EXCLAMATION_POINT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input>{}

pub struct UnaryExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UnaryExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for UnaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for UnaryExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_unaryExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for UnaryExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_unaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for UnaryExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for UnaryExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for UnaryExpressionContext<'input> {}

impl<'input> UnaryExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::UnaryExpressionContext(
				BaseParserRuleContext::copy_from(ctx,UnaryExpressionContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LiteralExpressionContext<'input> = BaseParserRuleContext<'input,LiteralExpressionContextExt<'input>>;

pub trait LiteralExpressionContextAttrs<'input>: qasm3ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(Identifier, 0)
	}
	/// Retrieves first TerminalNode corresponding to token BinaryIntegerLiteral
	/// Returns `None` if there is no child corresponding to token BinaryIntegerLiteral
	fn BinaryIntegerLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(BinaryIntegerLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OctalIntegerLiteral
	/// Returns `None` if there is no child corresponding to token OctalIntegerLiteral
	fn OctalIntegerLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(OctalIntegerLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token DecimalIntegerLiteral
	/// Returns `None` if there is no child corresponding to token DecimalIntegerLiteral
	fn DecimalIntegerLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(DecimalIntegerLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token HexIntegerLiteral
	/// Returns `None` if there is no child corresponding to token HexIntegerLiteral
	fn HexIntegerLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(HexIntegerLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token FloatLiteral
	/// Returns `None` if there is no child corresponding to token FloatLiteral
	fn FloatLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(FloatLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ImaginaryLiteral
	/// Returns `None` if there is no child corresponding to token ImaginaryLiteral
	fn ImaginaryLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(ImaginaryLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token BooleanLiteral
	/// Returns `None` if there is no child corresponding to token BooleanLiteral
	fn BooleanLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(BooleanLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token BitstringLiteral
	/// Returns `None` if there is no child corresponding to token BitstringLiteral
	fn BitstringLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(BitstringLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token TimingLiteral
	/// Returns `None` if there is no child corresponding to token TimingLiteral
	fn TimingLiteral(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(TimingLiteral, 0)
	}
	/// Retrieves first TerminalNode corresponding to token HardwareQubit
	/// Returns `None` if there is no child corresponding to token HardwareQubit
	fn HardwareQubit(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
		self.get_token(HardwareQubit, 0)
	}
}

impl<'input> LiteralExpressionContextAttrs<'input> for LiteralExpressionContext<'input>{}

pub struct LiteralExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LiteralExpressionContextExt<'a>}

impl<'input> qasm3ParserContext<'input> for LiteralExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for LiteralExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_literalExpression(self);
	}
	fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
		listener.exit_literalExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for LiteralExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_literalExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for LiteralExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for LiteralExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for LiteralExpressionContext<'input> {}

impl<'input> LiteralExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::LiteralExpressionContext(
				BaseParserRuleContext::copy_from(ctx,LiteralExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 70, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 70;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(471);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(43,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = ParenthesisExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(445);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(446);
					recog.expression_rec(0)?;

					recog.base.set_state(447);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = UnaryExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(449);
					if let ExpressionContextAll::UnaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
					_la = recog.base.input.la(1);
					if { !(((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)))) != 0)) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						if let ExpressionContextAll::UnaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
						ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(450);
					recog.expression_rec(15)?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = CastExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(453);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | DURATION | STRETCH 
						=> {
							{
							/*InvokeRule scalarType*/
							recog.base.set_state(451);
							recog.scalarType()?;

							}
						}

					 ARRAY 
						=> {
							{
							/*InvokeRule arrayType*/
							recog.base.set_state(452);
							recog.arrayType()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(455);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(456);
					recog.expression_rec(0)?;

					recog.base.set_state(457);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					let mut tmp = DurationofExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(459);
					recog.base.match_token(DURATIONOF,&mut recog.err_handler)?;

					recog.base.set_state(460);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule scope*/
					recog.base.set_state(461);
					recog.scope()?;

					recog.base.set_state(462);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = CallExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(464);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(465);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(467);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 30)) & !0x3f) == 0 && ((1usize << (_la - 30)) & ((1usize << (BOOL - 30)) | (1usize << (BIT - 30)) | (1usize << (INT - 30)) | (1usize << (UINT - 30)) | (1usize << (FLOAT - 30)) | (1usize << (ANGLE - 30)) | (1usize << (COMPLEX - 30)) | (1usize << (ARRAY - 30)) | (1usize << (DURATION - 30)) | (1usize << (STRETCH - 30)) | (1usize << (DURATIONOF - 30)) | (1usize << (BooleanLiteral - 30)) | (1usize << (LPAREN - 30)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
						{
						/*InvokeRule expressionList*/
						recog.base.set_state(466);
						recog.expressionList()?;

						}
					}

					recog.base.set_state(469);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					let mut tmp = LiteralExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(470);
					_la = recog.base.input.la(1);
					if { !(_la==BooleanLiteral || ((((_la - 85)) & !0x3f) == 0 && ((1usize << (_la - 85)) & ((1usize << (ImaginaryLiteral - 85)) | (1usize << (BinaryIntegerLiteral - 85)) | (1usize << (OctalIntegerLiteral - 85)) | (1usize << (DecimalIntegerLiteral - 85)) | (1usize << (HexIntegerLiteral - 85)) | (1usize << (Identifier - 85)) | (1usize << (HardwareQubit - 85)) | (1usize << (FloatLiteral - 85)) | (1usize << (TimingLiteral - 85)) | (1usize << (BitstringLiteral - 85)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(510);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(45,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(508);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(44,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = PowerExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(473);
							if !({recog.precpred(None, 16)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 16)".to_owned()), None))?;
							}
							recog.base.set_state(474);
							let tmp = recog.base.match_token(DOUBLE_ASTERISK,&mut recog.err_handler)?;
							if let ExpressionContextAll::PowerExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(475);
							recog.expression_rec(16)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MultiplicativeExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(476);
							if !({recog.precpred(None, 14)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 14)".to_owned()), None))?;
							}
							recog.base.set_state(477);
							if let ExpressionContextAll::MultiplicativeExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & ((1usize << (ASTERISK - 68)) | (1usize << (SLASH - 68)) | (1usize << (PERCENT - 68)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::MultiplicativeExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(478);
							recog.expression_rec(15)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AdditiveExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(479);
							if !({recog.precpred(None, 13)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 13)".to_owned()), None))?;
							}
							recog.base.set_state(480);
							if let ExpressionContextAll::AdditiveExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==PLUS || _la==MINUS) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::AdditiveExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(481);
							recog.expression_rec(14)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BitshiftExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(482);
							if !({recog.precpred(None, 12)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 12)".to_owned()), None))?;
							}
							recog.base.set_state(483);
							let tmp = recog.base.match_token(BitshiftOperator,&mut recog.err_handler)?;
							if let ExpressionContextAll::BitshiftExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(484);
							recog.expression_rec(13)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ComparisonExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(485);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(486);
							let tmp = recog.base.match_token(ComparisonOperator,&mut recog.err_handler)?;
							if let ExpressionContextAll::ComparisonExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(487);
							recog.expression_rec(12)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = EqualityExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(488);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(489);
							let tmp = recog.base.match_token(EqualityOperator,&mut recog.err_handler)?;
							if let ExpressionContextAll::EqualityExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(490);
							recog.expression_rec(11)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BitwiseAndExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(491);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(492);
							let tmp = recog.base.match_token(AMPERSAND,&mut recog.err_handler)?;
							if let ExpressionContextAll::BitwiseAndExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(493);
							recog.expression_rec(10)?;

							}
						}
					,
						8 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BitwiseXorExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(494);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(495);
							let tmp = recog.base.match_token(CARET,&mut recog.err_handler)?;
							if let ExpressionContextAll::BitwiseXorExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(496);
							recog.expression_rec(9)?;

							}
						}
					,
						9 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BitwiseOrExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(497);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(498);
							let tmp = recog.base.match_token(PIPE,&mut recog.err_handler)?;
							if let ExpressionContextAll::BitwiseOrExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(499);
							recog.expression_rec(8)?;

							}
						}
					,
						10 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = LogicalAndExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(500);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(501);
							let tmp = recog.base.match_token(DOUBLE_AMPERSAND,&mut recog.err_handler)?;
							if let ExpressionContextAll::LogicalAndExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(502);
							recog.expression_rec(7)?;

							}
						}
					,
						11 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = LogicalOrExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(503);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(504);
							let tmp = recog.base.match_token(DOUBLE_PIPE,&mut recog.err_handler)?;
							if let ExpressionContextAll::LogicalOrExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expression*/
							recog.base.set_state(505);
							recog.expression_rec(6)?;

							}
						}
					,
						12 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = IndexExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(506);
							if !({recog.precpred(None, 17)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 17)".to_owned()), None))?;
							}
							/*InvokeRule indexOperator*/
							recog.base.set_state(507);
							recog.indexOperator()?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(512);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(45,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- aliasExpression ----------------
pub type AliasExpressionContextAll<'input> = AliasExpressionContext<'input>;


pub type AliasExpressionContext<'input> = BaseParserRuleContext<'input,AliasExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AliasExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for AliasExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for AliasExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_aliasExpression(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_aliasExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for AliasExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_aliasExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AliasExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aliasExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aliasExpression }
}
antlr_rust::tid!{AliasExpressionContextExt<'a>}

impl<'input> AliasExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AliasExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AliasExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AliasExpressionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<AliasExpressionContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOUBLE_PLUS in current rule
fn DOUBLE_PLUS_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOUBLE_PLUS, starting from 0.
/// Returns `None` if number of children corresponding to token DOUBLE_PLUS is less or equal than `i`.
fn DOUBLE_PLUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE_PLUS, i)
}

}

impl<'input> AliasExpressionContextAttrs<'input> for AliasExpressionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn aliasExpression(&mut self,)
	-> Result<Rc<AliasExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AliasExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_aliasExpression);
        let mut _localctx: Rc<AliasExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(513);
			recog.expression_rec(0)?;

			recog.base.set_state(518);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DOUBLE_PLUS {
				{
				{
				recog.base.set_state(514);
				recog.base.match_token(DOUBLE_PLUS,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(515);
				recog.expression_rec(0)?;

				}
				}
				recog.base.set_state(520);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declarationExpression ----------------
pub type DeclarationExpressionContextAll<'input> = DeclarationExpressionContext<'input>;


pub type DeclarationExpressionContext<'input> = BaseParserRuleContext<'input,DeclarationExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DeclarationExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DeclarationExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_declarationExpression(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_declarationExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DeclarationExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_declarationExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationExpression }
}
antlr_rust::tid!{DeclarationExpressionContextExt<'a>}

impl<'input> DeclarationExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationExpressionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DeclarationExpressionContextExt<'input>>{

fn arrayLiteral(&self) -> Option<Rc<ArrayLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn measureExpression(&self) -> Option<Rc<MeasureExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationExpressionContextAttrs<'input> for DeclarationExpressionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declarationExpression(&mut self,)
	-> Result<Rc<DeclarationExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_declarationExpression);
        let mut _localctx: Rc<DeclarationExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(524);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LBRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule arrayLiteral*/
					recog.base.set_state(521);
					recog.arrayLiteral()?;

					}
				}

			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | ARRAY | DURATION |
			 STRETCH | DURATIONOF | BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT |
			 ImaginaryLiteral | BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
			 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
			 BitstringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(522);
					recog.expression_rec(0)?;

					}
				}

			 MEASURE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule measureExpression*/
					recog.base.set_state(523);
					recog.measureExpression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- measureExpression ----------------
pub type MeasureExpressionContextAll<'input> = MeasureExpressionContext<'input>;


pub type MeasureExpressionContext<'input> = BaseParserRuleContext<'input,MeasureExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct MeasureExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for MeasureExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for MeasureExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_measureExpression(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_measureExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for MeasureExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_measureExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for MeasureExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_measureExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_measureExpression }
}
antlr_rust::tid!{MeasureExpressionContextExt<'a>}

impl<'input> MeasureExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MeasureExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MeasureExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MeasureExpressionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<MeasureExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MEASURE
/// Returns `None` if there is no child corresponding to token MEASURE
fn MEASURE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(MEASURE, 0)
}
fn gateOperand(&self) -> Option<Rc<GateOperandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MeasureExpressionContextAttrs<'input> for MeasureExpressionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn measureExpression(&mut self,)
	-> Result<Rc<MeasureExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MeasureExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_measureExpression);
        let mut _localctx: Rc<MeasureExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(526);
			recog.base.match_token(MEASURE,&mut recog.err_handler)?;

			/*InvokeRule gateOperand*/
			recog.base.set_state(527);
			recog.gateOperand()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- rangeExpression ----------------
pub type RangeExpressionContextAll<'input> = RangeExpressionContext<'input>;


pub type RangeExpressionContext<'input> = BaseParserRuleContext<'input,RangeExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct RangeExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for RangeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for RangeExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rangeExpression(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_rangeExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for RangeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_rangeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for RangeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rangeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rangeExpression }
}
antlr_rust::tid!{RangeExpressionContextExt<'a>}

impl<'input> RangeExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RangeExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RangeExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RangeExpressionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<RangeExpressionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RangeExpressionContextAttrs<'input> for RangeExpressionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rangeExpression(&mut self,)
	-> Result<Rc<RangeExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RangeExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_rangeExpression);
        let mut _localctx: Rc<RangeExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(530);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 30)) & !0x3f) == 0 && ((1usize << (_la - 30)) & ((1usize << (BOOL - 30)) | (1usize << (BIT - 30)) | (1usize << (INT - 30)) | (1usize << (UINT - 30)) | (1usize << (FLOAT - 30)) | (1usize << (ANGLE - 30)) | (1usize << (COMPLEX - 30)) | (1usize << (ARRAY - 30)) | (1usize << (DURATION - 30)) | (1usize << (STRETCH - 30)) | (1usize << (DURATIONOF - 30)) | (1usize << (BooleanLiteral - 30)) | (1usize << (LPAREN - 30)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(529);
				recog.expression_rec(0)?;

				}
			}

			recog.base.set_state(532);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(534);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 30)) & !0x3f) == 0 && ((1usize << (_la - 30)) & ((1usize << (BOOL - 30)) | (1usize << (BIT - 30)) | (1usize << (INT - 30)) | (1usize << (UINT - 30)) | (1usize << (FLOAT - 30)) | (1usize << (ANGLE - 30)) | (1usize << (COMPLEX - 30)) | (1usize << (ARRAY - 30)) | (1usize << (DURATION - 30)) | (1usize << (STRETCH - 30)) | (1usize << (DURATIONOF - 30)) | (1usize << (BooleanLiteral - 30)) | (1usize << (LPAREN - 30)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (MINUS - 67)) | (1usize << (TILDE - 67)) | (1usize << (EXCLAMATION_POINT - 67)) | (1usize << (ImaginaryLiteral - 67)) | (1usize << (BinaryIntegerLiteral - 67)) | (1usize << (OctalIntegerLiteral - 67)) | (1usize << (DecimalIntegerLiteral - 67)) | (1usize << (HexIntegerLiteral - 67)) | (1usize << (Identifier - 67)) | (1usize << (HardwareQubit - 67)) | (1usize << (FloatLiteral - 67)) | (1usize << (TimingLiteral - 67)) | (1usize << (BitstringLiteral - 67)))) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(533);
				recog.expression_rec(0)?;

				}
			}

			recog.base.set_state(538);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COLON {
				{
				recog.base.set_state(536);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(537);
				recog.expression_rec(0)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- setExpression ----------------
pub type SetExpressionContextAll<'input> = SetExpressionContext<'input>;


pub type SetExpressionContext<'input> = BaseParserRuleContext<'input,SetExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct SetExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for SetExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for SetExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setExpression(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_setExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for SetExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_setExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setExpression }
}
antlr_rust::tid!{SetExpressionContextExt<'a>}

impl<'input> SetExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetExpressionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<SetExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> SetExpressionContextAttrs<'input> for SetExpressionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setExpression(&mut self,)
	-> Result<Rc<SetExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_setExpression);
        let mut _localctx: Rc<SetExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(540);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(541);
			recog.expression_rec(0)?;

			recog.base.set_state(546);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(51,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(542);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(543);
					recog.expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(548);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(51,&mut recog.base)?;
			}
			recog.base.set_state(550);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(549);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(552);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arrayLiteral ----------------
pub type ArrayLiteralContextAll<'input> = ArrayLiteralContext<'input>;


pub type ArrayLiteralContext<'input> = BaseParserRuleContext<'input,ArrayLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ArrayLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ArrayLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arrayLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_arrayLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ArrayLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_arrayLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayLiteral }
}
antlr_rust::tid!{ArrayLiteralContextExt<'a>}

impl<'input> ArrayLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayLiteralContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ArrayLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn arrayLiteral_all(&self) ->  Vec<Rc<ArrayLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn arrayLiteral(&self, i: usize) -> Option<Rc<ArrayLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ArrayLiteralContextAttrs<'input> for ArrayLiteralContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arrayLiteral(&mut self,)
	-> Result<Rc<ArrayLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_arrayLiteral);
        let mut _localctx: Rc<ArrayLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(554);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(557);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | ARRAY | DURATION |
			 STRETCH | DURATIONOF | BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT |
			 ImaginaryLiteral | BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
			 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
			 BitstringLiteral 
				=> {
					{
					/*InvokeRule expression*/
					recog.base.set_state(555);
					recog.expression_rec(0)?;

					}
				}

			 LBRACE 
				=> {
					{
					/*InvokeRule arrayLiteral*/
					recog.base.set_state(556);
					recog.arrayLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(566);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(55,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(559);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					recog.base.set_state(562);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | ARRAY | DURATION |
					 STRETCH | DURATIONOF | BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT |
					 ImaginaryLiteral | BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
					 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
					 BitstringLiteral 
						=> {
							{
							/*InvokeRule expression*/
							recog.base.set_state(560);
							recog.expression_rec(0)?;

							}
						}

					 LBRACE 
						=> {
							{
							/*InvokeRule arrayLiteral*/
							recog.base.set_state(561);
							recog.arrayLiteral()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
					} 
				}
				recog.base.set_state(568);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(55,&mut recog.base)?;
			}
			recog.base.set_state(570);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(569);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(572);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- indexOperator ----------------
pub type IndexOperatorContextAll<'input> = IndexOperatorContext<'input>;


pub type IndexOperatorContext<'input> = BaseParserRuleContext<'input,IndexOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct IndexOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for IndexOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for IndexOperatorContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_indexOperator(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_indexOperator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for IndexOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_indexOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for IndexOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_indexOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_indexOperator }
}
antlr_rust::tid!{IndexOperatorContextExt<'a>}

impl<'input> IndexOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IndexOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IndexOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IndexOperatorContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<IndexOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
fn setExpression(&self) -> Option<Rc<SetExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn rangeExpression_all(&self) ->  Vec<Rc<RangeExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn rangeExpression(&self, i: usize) -> Option<Rc<RangeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> IndexOperatorContextAttrs<'input> for IndexOperatorContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn indexOperator(&mut self,)
	-> Result<Rc<IndexOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IndexOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_indexOperator);
        let mut _localctx: Rc<IndexOperatorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(574);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			recog.base.set_state(593);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LBRACE 
				=> {
					{
					/*InvokeRule setExpression*/
					recog.base.set_state(575);
					recog.setExpression()?;

					}
				}

			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | ARRAY | DURATION |
			 STRETCH | DURATIONOF | BooleanLiteral | LPAREN | COLON | MINUS | TILDE |
			 EXCLAMATION_POINT | ImaginaryLiteral | BinaryIntegerLiteral | OctalIntegerLiteral |
			 DecimalIntegerLiteral | HexIntegerLiteral | Identifier | HardwareQubit |
			 FloatLiteral | TimingLiteral | BitstringLiteral 
				=> {
					{
					recog.base.set_state(578);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule expression*/
							recog.base.set_state(576);
							recog.expression_rec(0)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule rangeExpression*/
							recog.base.set_state(577);
							recog.rangeExpression()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(587);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(59,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(580);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							recog.base.set_state(583);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(58,&mut recog.base)? {
								1 =>{
									{
									/*InvokeRule expression*/
									recog.base.set_state(581);
									recog.expression_rec(0)?;

									}
								}
							,
								2 =>{
									{
									/*InvokeRule rangeExpression*/
									recog.base.set_state(582);
									recog.rangeExpression()?;

									}
								}

								_ => {}
							}
							}
							} 
						}
						recog.base.set_state(589);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(59,&mut recog.base)?;
					}
					recog.base.set_state(591);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(590);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(595);
			recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- indexedIdentifier ----------------
pub type IndexedIdentifierContextAll<'input> = IndexedIdentifierContext<'input>;


pub type IndexedIdentifierContext<'input> = BaseParserRuleContext<'input,IndexedIdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct IndexedIdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for IndexedIdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for IndexedIdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_indexedIdentifier(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_indexedIdentifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for IndexedIdentifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_indexedIdentifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for IndexedIdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_indexedIdentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_indexedIdentifier }
}
antlr_rust::tid!{IndexedIdentifierContextExt<'a>}

impl<'input> IndexedIdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IndexedIdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IndexedIdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IndexedIdentifierContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<IndexedIdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn indexOperator_all(&self) ->  Vec<Rc<IndexOperatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn indexOperator(&self, i: usize) -> Option<Rc<IndexOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IndexedIdentifierContextAttrs<'input> for IndexedIdentifierContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn indexedIdentifier(&mut self,)
	-> Result<Rc<IndexedIdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IndexedIdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_indexedIdentifier);
        let mut _localctx: Rc<IndexedIdentifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(597);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(601);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==LBRACKET {
				{
				{
				/*InvokeRule indexOperator*/
				recog.base.set_state(598);
				recog.indexOperator()?;

				}
				}
				recog.base.set_state(603);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- returnSignature ----------------
pub type ReturnSignatureContextAll<'input> = ReturnSignatureContext<'input>;


pub type ReturnSignatureContext<'input> = BaseParserRuleContext<'input,ReturnSignatureContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnSignatureContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ReturnSignatureContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ReturnSignatureContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnSignature(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_returnSignature(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ReturnSignatureContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_returnSignature(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnSignatureContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnSignature }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnSignature }
}
antlr_rust::tid!{ReturnSignatureContextExt<'a>}

impl<'input> ReturnSignatureContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnSignatureContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnSignatureContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnSignatureContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ReturnSignatureContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnSignatureContextAttrs<'input> for ReturnSignatureContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnSignature(&mut self,)
	-> Result<Rc<ReturnSignatureContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnSignatureContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_returnSignature);
        let mut _localctx: Rc<ReturnSignatureContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(604);
			recog.base.match_token(ARROW,&mut recog.err_handler)?;

			/*InvokeRule scalarType*/
			recog.base.set_state(605);
			recog.scalarType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- gateModifier ----------------
pub type GateModifierContextAll<'input> = GateModifierContext<'input>;


pub type GateModifierContext<'input> = BaseParserRuleContext<'input,GateModifierContextExt<'input>>;

#[derive(Clone)]
pub struct GateModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for GateModifierContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for GateModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_gateModifier(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_gateModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for GateModifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_gateModifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for GateModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gateModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gateModifier }
}
antlr_rust::tid!{GateModifierContextExt<'a>}

impl<'input> GateModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GateModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GateModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GateModifierContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<GateModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AT
/// Returns `None` if there is no child corresponding to token AT
fn AT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(AT, 0)
}
/// Retrieves first TerminalNode corresponding to token INV
/// Returns `None` if there is no child corresponding to token INV
fn INV(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(INV, 0)
}
/// Retrieves first TerminalNode corresponding to token POW
/// Returns `None` if there is no child corresponding to token POW
fn POW(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(POW, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token CTRL
/// Returns `None` if there is no child corresponding to token CTRL
fn CTRL(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CTRL, 0)
}
/// Retrieves first TerminalNode corresponding to token NEGCTRL
/// Returns `None` if there is no child corresponding to token NEGCTRL
fn NEGCTRL(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(NEGCTRL, 0)
}

}

impl<'input> GateModifierContextAttrs<'input> for GateModifierContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn gateModifier(&mut self,)
	-> Result<Rc<GateModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GateModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_gateModifier);
        let mut _localctx: Rc<GateModifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(620);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INV 
				=> {
					{
					recog.base.set_state(607);
					recog.base.match_token(INV,&mut recog.err_handler)?;

					}
				}

			 POW 
				=> {
					{
					recog.base.set_state(608);
					recog.base.match_token(POW,&mut recog.err_handler)?;

					recog.base.set_state(609);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(610);
					recog.expression_rec(0)?;

					recog.base.set_state(611);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 CTRL | NEGCTRL 
				=> {
					{
					recog.base.set_state(613);
					_la = recog.base.input.la(1);
					if { !(_la==CTRL || _la==NEGCTRL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(618);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						recog.base.set_state(614);
						recog.base.match_token(LPAREN,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(615);
						recog.expression_rec(0)?;

						recog.base.set_state(616);
						recog.base.match_token(RPAREN,&mut recog.err_handler)?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(622);
			recog.base.match_token(AT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scalarType ----------------
pub type ScalarTypeContextAll<'input> = ScalarTypeContext<'input>;


pub type ScalarTypeContext<'input> = BaseParserRuleContext<'input,ScalarTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ScalarTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ScalarTypeContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ScalarTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_scalarType(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_scalarType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ScalarTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_scalarType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScalarTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}
antlr_rust::tid!{ScalarTypeContextExt<'a>}

impl<'input> ScalarTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScalarTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScalarTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScalarTypeContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ScalarTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BIT
/// Returns `None` if there is no child corresponding to token BIT
fn BIT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(BIT, 0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token UINT
/// Returns `None` if there is no child corresponding to token UINT
fn UINT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(UINT, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, 0)
}
/// Retrieves first TerminalNode corresponding to token ANGLE
/// Returns `None` if there is no child corresponding to token ANGLE
fn ANGLE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(ANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL
/// Returns `None` if there is no child corresponding to token BOOL
fn BOOL(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(BOOL, 0)
}
/// Retrieves first TerminalNode corresponding to token DURATION
/// Returns `None` if there is no child corresponding to token DURATION
fn DURATION(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DURATION, 0)
}
/// Retrieves first TerminalNode corresponding to token STRETCH
/// Returns `None` if there is no child corresponding to token STRETCH
fn STRETCH(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(STRETCH, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPLEX
/// Returns `None` if there is no child corresponding to token COMPLEX
fn COMPLEX(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMPLEX, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}

}

impl<'input> ScalarTypeContextAttrs<'input> for ScalarTypeContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scalarType(&mut self,)
	-> Result<Rc<ScalarTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScalarTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_scalarType);
        let mut _localctx: Rc<ScalarTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(654);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BIT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(624);
					recog.base.match_token(BIT,&mut recog.err_handler)?;

					recog.base.set_state(626);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(625);
						recog.designator()?;

						}
					}

					}
				}

			 INT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(628);
					recog.base.match_token(INT,&mut recog.err_handler)?;

					recog.base.set_state(630);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(629);
						recog.designator()?;

						}
					}

					}
				}

			 UINT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(632);
					recog.base.match_token(UINT,&mut recog.err_handler)?;

					recog.base.set_state(634);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(633);
						recog.designator()?;

						}
					}

					}
				}

			 FLOAT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(636);
					recog.base.match_token(FLOAT,&mut recog.err_handler)?;

					recog.base.set_state(638);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(637);
						recog.designator()?;

						}
					}

					}
				}

			 ANGLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(640);
					recog.base.match_token(ANGLE,&mut recog.err_handler)?;

					recog.base.set_state(642);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(641);
						recog.designator()?;

						}
					}

					}
				}

			 BOOL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(644);
					recog.base.match_token(BOOL,&mut recog.err_handler)?;

					}
				}

			 DURATION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(645);
					recog.base.match_token(DURATION,&mut recog.err_handler)?;

					}
				}

			 STRETCH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(646);
					recog.base.match_token(STRETCH,&mut recog.err_handler)?;

					}
				}

			 COMPLEX 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(647);
					recog.base.match_token(COMPLEX,&mut recog.err_handler)?;

					recog.base.set_state(652);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						recog.base.set_state(648);
						recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

						/*InvokeRule scalarType*/
						recog.base.set_state(649);
						recog.scalarType()?;

						recog.base.set_state(650);
						recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- qubitType ----------------
pub type QubitTypeContextAll<'input> = QubitTypeContext<'input>;


pub type QubitTypeContext<'input> = BaseParserRuleContext<'input,QubitTypeContextExt<'input>>;

#[derive(Clone)]
pub struct QubitTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for QubitTypeContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for QubitTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qubitType(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_qubitType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for QubitTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_qubitType(self);
	}
}

impl<'input> CustomRuleContext<'input> for QubitTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qubitType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qubitType }
}
antlr_rust::tid!{QubitTypeContextExt<'a>}

impl<'input> QubitTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QubitTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QubitTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QubitTypeContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<QubitTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token QUBIT
/// Returns `None` if there is no child corresponding to token QUBIT
fn QUBIT(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(QUBIT, 0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> QubitTypeContextAttrs<'input> for QubitTypeContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qubitType(&mut self,)
	-> Result<Rc<QubitTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QubitTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_qubitType);
        let mut _localctx: Rc<QubitTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(656);
			recog.base.match_token(QUBIT,&mut recog.err_handler)?;

			recog.base.set_state(658);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACKET {
				{
				/*InvokeRule designator*/
				recog.base.set_state(657);
				recog.designator()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arrayType ----------------
pub type ArrayTypeContextAll<'input> = ArrayTypeContext<'input>;


pub type ArrayTypeContext<'input> = BaseParserRuleContext<'input,ArrayTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ArrayTypeContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ArrayTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arrayType(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_arrayType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ArrayTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_arrayType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayType }
}
antlr_rust::tid!{ArrayTypeContextExt<'a>}

impl<'input> ArrayTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayTypeContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ArrayTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ARRAY
/// Returns `None` if there is no child corresponding to token ARRAY
fn ARRAY(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(ARRAY, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}

}

impl<'input> ArrayTypeContextAttrs<'input> for ArrayTypeContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arrayType(&mut self,)
	-> Result<Rc<ArrayTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_arrayType);
        let mut _localctx: Rc<ArrayTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(660);
			recog.base.match_token(ARRAY,&mut recog.err_handler)?;

			recog.base.set_state(661);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			/*InvokeRule scalarType*/
			recog.base.set_state(662);
			recog.scalarType()?;

			recog.base.set_state(663);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			/*InvokeRule expressionList*/
			recog.base.set_state(664);
			recog.expressionList()?;

			recog.base.set_state(665);
			recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arrayReferenceType ----------------
pub type ArrayReferenceTypeContextAll<'input> = ArrayReferenceTypeContext<'input>;


pub type ArrayReferenceTypeContext<'input> = BaseParserRuleContext<'input,ArrayReferenceTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayReferenceTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ArrayReferenceTypeContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ArrayReferenceTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arrayReferenceType(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_arrayReferenceType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ArrayReferenceTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_arrayReferenceType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayReferenceTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayReferenceType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayReferenceType }
}
antlr_rust::tid!{ArrayReferenceTypeContextExt<'a>}

impl<'input> ArrayReferenceTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayReferenceTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayReferenceTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayReferenceTypeContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ArrayReferenceTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ARRAY
/// Returns `None` if there is no child corresponding to token ARRAY
fn ARRAY(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(ARRAY, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token READONLY
/// Returns `None` if there is no child corresponding to token READONLY
fn READONLY(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(READONLY, 0)
}
/// Retrieves first TerminalNode corresponding to token MUTABLE
/// Returns `None` if there is no child corresponding to token MUTABLE
fn MUTABLE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(MUTABLE, 0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DIM
/// Returns `None` if there is no child corresponding to token DIM
fn DIM(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DIM, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ArrayReferenceTypeContextAttrs<'input> for ArrayReferenceTypeContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arrayReferenceType(&mut self,)
	-> Result<Rc<ArrayReferenceTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayReferenceTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_arrayReferenceType);
        let mut _localctx: Rc<ArrayReferenceTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(667);
			_la = recog.base.input.la(1);
			if { !(_la==READONLY || _la==MUTABLE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(668);
			recog.base.match_token(ARRAY,&mut recog.err_handler)?;

			recog.base.set_state(669);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			/*InvokeRule scalarType*/
			recog.base.set_state(670);
			recog.scalarType()?;

			recog.base.set_state(671);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			recog.base.set_state(676);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | ARRAY | DURATION |
			 STRETCH | DURATIONOF | BooleanLiteral | LPAREN | MINUS | TILDE | EXCLAMATION_POINT |
			 ImaginaryLiteral | BinaryIntegerLiteral | OctalIntegerLiteral | DecimalIntegerLiteral |
			 HexIntegerLiteral | Identifier | HardwareQubit | FloatLiteral | TimingLiteral |
			 BitstringLiteral 
				=> {
					{
					/*InvokeRule expressionList*/
					recog.base.set_state(672);
					recog.expressionList()?;

					}
				}

			 DIM 
				=> {
					{
					recog.base.set_state(673);
					recog.base.match_token(DIM,&mut recog.err_handler)?;

					recog.base.set_state(674);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(675);
					recog.expression_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(678);
			recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- designator ----------------
pub type DesignatorContextAll<'input> = DesignatorContext<'input>;


pub type DesignatorContext<'input> = BaseParserRuleContext<'input,DesignatorContextExt<'input>>;

#[derive(Clone)]
pub struct DesignatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DesignatorContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DesignatorContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_designator(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_designator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DesignatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_designator(self);
	}
}

impl<'input> CustomRuleContext<'input> for DesignatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_designator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_designator }
}
antlr_rust::tid!{DesignatorContextExt<'a>}

impl<'input> DesignatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DesignatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DesignatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DesignatorContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DesignatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACKET
/// Returns `None` if there is no child corresponding to token LBRACKET
fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(LBRACKET, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACKET
/// Returns `None` if there is no child corresponding to token RBRACKET
fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RBRACKET, 0)
}

}

impl<'input> DesignatorContextAttrs<'input> for DesignatorContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn designator(&mut self,)
	-> Result<Rc<DesignatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DesignatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_designator);
        let mut _localctx: Rc<DesignatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(680);
			recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(681);
			recog.expression_rec(0)?;

			recog.base.set_state(682);
			recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defcalTarget ----------------
pub type DefcalTargetContextAll<'input> = DefcalTargetContext<'input>;


pub type DefcalTargetContext<'input> = BaseParserRuleContext<'input,DefcalTargetContextExt<'input>>;

#[derive(Clone)]
pub struct DefcalTargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DefcalTargetContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DefcalTargetContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defcalTarget(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_defcalTarget(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DefcalTargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_defcalTarget(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefcalTargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defcalTarget }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defcalTarget }
}
antlr_rust::tid!{DefcalTargetContextExt<'a>}

impl<'input> DefcalTargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefcalTargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefcalTargetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefcalTargetContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DefcalTargetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MEASURE
/// Returns `None` if there is no child corresponding to token MEASURE
fn MEASURE(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(MEASURE, 0)
}
/// Retrieves first TerminalNode corresponding to token RESET
/// Returns `None` if there is no child corresponding to token RESET
fn RESET(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(RESET, 0)
}
/// Retrieves first TerminalNode corresponding to token DELAY
/// Returns `None` if there is no child corresponding to token DELAY
fn DELAY(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(DELAY, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> DefcalTargetContextAttrs<'input> for DefcalTargetContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defcalTarget(&mut self,)
	-> Result<Rc<DefcalTargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefcalTargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_defcalTarget);
        let mut _localctx: Rc<DefcalTargetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(684);
			_la = recog.base.input.la(1);
			if { !(((((_la - 48)) & !0x3f) == 0 && ((1usize << (_la - 48)) & ((1usize << (DELAY - 48)) | (1usize << (RESET - 48)) | (1usize << (MEASURE - 48)))) != 0) || _la==Identifier) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defcalArgumentDefinition ----------------
pub type DefcalArgumentDefinitionContextAll<'input> = DefcalArgumentDefinitionContext<'input>;


pub type DefcalArgumentDefinitionContext<'input> = BaseParserRuleContext<'input,DefcalArgumentDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct DefcalArgumentDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DefcalArgumentDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DefcalArgumentDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defcalArgumentDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_defcalArgumentDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DefcalArgumentDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_defcalArgumentDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefcalArgumentDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defcalArgumentDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defcalArgumentDefinition }
}
antlr_rust::tid!{DefcalArgumentDefinitionContextExt<'a>}

impl<'input> DefcalArgumentDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefcalArgumentDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefcalArgumentDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefcalArgumentDefinitionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DefcalArgumentDefinitionContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn argumentDefinition(&self) -> Option<Rc<ArgumentDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DefcalArgumentDefinitionContextAttrs<'input> for DefcalArgumentDefinitionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defcalArgumentDefinition(&mut self,)
	-> Result<Rc<DefcalArgumentDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefcalArgumentDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_defcalArgumentDefinition);
        let mut _localctx: Rc<DefcalArgumentDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(688);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(74,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(686);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule argumentDefinition*/
					recog.base.set_state(687);
					recog.argumentDefinition()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defcalOperand ----------------
pub type DefcalOperandContextAll<'input> = DefcalOperandContext<'input>;


pub type DefcalOperandContext<'input> = BaseParserRuleContext<'input,DefcalOperandContextExt<'input>>;

#[derive(Clone)]
pub struct DefcalOperandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DefcalOperandContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DefcalOperandContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defcalOperand(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_defcalOperand(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DefcalOperandContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_defcalOperand(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefcalOperandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defcalOperand }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defcalOperand }
}
antlr_rust::tid!{DefcalOperandContextExt<'a>}

impl<'input> DefcalOperandContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefcalOperandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefcalOperandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefcalOperandContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DefcalOperandContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HardwareQubit
/// Returns `None` if there is no child corresponding to token HardwareQubit
fn HardwareQubit(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(HardwareQubit, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> DefcalOperandContextAttrs<'input> for DefcalOperandContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defcalOperand(&mut self,)
	-> Result<Rc<DefcalOperandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefcalOperandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_defcalOperand);
        let mut _localctx: Rc<DefcalOperandContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(690);
			_la = recog.base.input.la(1);
			if { !(_la==Identifier || _la==HardwareQubit) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- gateOperand ----------------
pub type GateOperandContextAll<'input> = GateOperandContext<'input>;


pub type GateOperandContext<'input> = BaseParserRuleContext<'input,GateOperandContextExt<'input>>;

#[derive(Clone)]
pub struct GateOperandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for GateOperandContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for GateOperandContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_gateOperand(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_gateOperand(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for GateOperandContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_gateOperand(self);
	}
}

impl<'input> CustomRuleContext<'input> for GateOperandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gateOperand }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gateOperand }
}
antlr_rust::tid!{GateOperandContextExt<'a>}

impl<'input> GateOperandContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GateOperandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GateOperandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GateOperandContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<GateOperandContextExt<'input>>{

fn indexedIdentifier(&self) -> Option<Rc<IndexedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token HardwareQubit
/// Returns `None` if there is no child corresponding to token HardwareQubit
fn HardwareQubit(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(HardwareQubit, 0)
}

}

impl<'input> GateOperandContextAttrs<'input> for GateOperandContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn gateOperand(&mut self,)
	-> Result<Rc<GateOperandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GateOperandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_gateOperand);
        let mut _localctx: Rc<GateOperandContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(694);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule indexedIdentifier*/
					recog.base.set_state(692);
					recog.indexedIdentifier()?;

					}
				}

			 HardwareQubit 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(693);
					recog.base.match_token(HardwareQubit,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- externArgument ----------------
pub type ExternArgumentContextAll<'input> = ExternArgumentContext<'input>;


pub type ExternArgumentContext<'input> = BaseParserRuleContext<'input,ExternArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct ExternArgumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ExternArgumentContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ExternArgumentContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_externArgument(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_externArgument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ExternArgumentContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_externArgument(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExternArgumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externArgument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externArgument }
}
antlr_rust::tid!{ExternArgumentContextExt<'a>}

impl<'input> ExternArgumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExternArgumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExternArgumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExternArgumentContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ExternArgumentContextExt<'input>>{

fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arrayReferenceType(&self) -> Option<Rc<ArrayReferenceTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CREG
/// Returns `None` if there is no child corresponding to token CREG
fn CREG(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CREG, 0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExternArgumentContextAttrs<'input> for ExternArgumentContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn externArgument(&mut self,)
	-> Result<Rc<ExternArgumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExternArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_externArgument);
        let mut _localctx: Rc<ExternArgumentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(702);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | DURATION | STRETCH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule scalarType*/
					recog.base.set_state(696);
					recog.scalarType()?;

					}
				}

			 READONLY | MUTABLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule arrayReferenceType*/
					recog.base.set_state(697);
					recog.arrayReferenceType()?;

					}
				}

			 CREG 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(698);
					recog.base.match_token(CREG,&mut recog.err_handler)?;

					recog.base.set_state(700);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(699);
						recog.designator()?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- argumentDefinition ----------------
pub type ArgumentDefinitionContextAll<'input> = ArgumentDefinitionContext<'input>;


pub type ArgumentDefinitionContext<'input> = BaseParserRuleContext<'input,ArgumentDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ArgumentDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ArgumentDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_argumentDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_argumentDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ArgumentDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_argumentDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArgumentDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_argumentDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_argumentDefinition }
}
antlr_rust::tid!{ArgumentDefinitionContextExt<'a>}

impl<'input> ArgumentDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgumentDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentDefinitionContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ArgumentDefinitionContextExt<'input>>{

fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn qubitType(&self) -> Option<Rc<QubitTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CREG
/// Returns `None` if there is no child corresponding to token CREG
fn CREG(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(CREG, 0)
}
/// Retrieves first TerminalNode corresponding to token QREG
/// Returns `None` if there is no child corresponding to token QREG
fn QREG(&self) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(QREG, 0)
}
fn designator(&self) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arrayReferenceType(&self) -> Option<Rc<ArrayReferenceTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ArgumentDefinitionContextAttrs<'input> for ArgumentDefinitionContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn argumentDefinition(&mut self,)
	-> Result<Rc<ArgumentDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_argumentDefinition);
        let mut _localctx: Rc<ArgumentDefinitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(718);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BIT | INT | UINT | FLOAT | ANGLE | COMPLEX | DURATION | STRETCH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule scalarType*/
					recog.base.set_state(704);
					recog.scalarType()?;

					recog.base.set_state(705);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

			 QUBIT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule qubitType*/
					recog.base.set_state(707);
					recog.qubitType()?;

					recog.base.set_state(708);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

			 QREG | CREG 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(710);
					_la = recog.base.input.la(1);
					if { !(_la==QREG || _la==CREG) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(711);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(713);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACKET {
						{
						/*InvokeRule designator*/
						recog.base.set_state(712);
						recog.designator()?;

						}
					}

					}
				}

			 READONLY | MUTABLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule arrayReferenceType*/
					recog.base.set_state(715);
					recog.arrayReferenceType()?;

					recog.base.set_state(716);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- argumentDefinitionList ----------------
pub type ArgumentDefinitionListContextAll<'input> = ArgumentDefinitionListContext<'input>;


pub type ArgumentDefinitionListContext<'input> = BaseParserRuleContext<'input,ArgumentDefinitionListContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentDefinitionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ArgumentDefinitionListContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ArgumentDefinitionListContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_argumentDefinitionList(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_argumentDefinitionList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ArgumentDefinitionListContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_argumentDefinitionList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArgumentDefinitionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_argumentDefinitionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_argumentDefinitionList }
}
antlr_rust::tid!{ArgumentDefinitionListContextExt<'a>}

impl<'input> ArgumentDefinitionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgumentDefinitionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentDefinitionListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentDefinitionListContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ArgumentDefinitionListContextExt<'input>>{

fn argumentDefinition_all(&self) ->  Vec<Rc<ArgumentDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn argumentDefinition(&self, i: usize) -> Option<Rc<ArgumentDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ArgumentDefinitionListContextAttrs<'input> for ArgumentDefinitionListContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn argumentDefinitionList(&mut self,)
	-> Result<Rc<ArgumentDefinitionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentDefinitionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_argumentDefinitionList);
        let mut _localctx: Rc<ArgumentDefinitionListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule argumentDefinition*/
			recog.base.set_state(720);
			recog.argumentDefinition()?;

			recog.base.set_state(725);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(80,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(721);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule argumentDefinition*/
					recog.base.set_state(722);
					recog.argumentDefinition()?;

					}
					} 
				}
				recog.base.set_state(727);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(80,&mut recog.base)?;
			}
			recog.base.set_state(729);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(728);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defcalArgumentDefinitionList ----------------
pub type DefcalArgumentDefinitionListContextAll<'input> = DefcalArgumentDefinitionListContext<'input>;


pub type DefcalArgumentDefinitionListContext<'input> = BaseParserRuleContext<'input,DefcalArgumentDefinitionListContextExt<'input>>;

#[derive(Clone)]
pub struct DefcalArgumentDefinitionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DefcalArgumentDefinitionListContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DefcalArgumentDefinitionListContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defcalArgumentDefinitionList(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_defcalArgumentDefinitionList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DefcalArgumentDefinitionListContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_defcalArgumentDefinitionList(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefcalArgumentDefinitionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defcalArgumentDefinitionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defcalArgumentDefinitionList }
}
antlr_rust::tid!{DefcalArgumentDefinitionListContextExt<'a>}

impl<'input> DefcalArgumentDefinitionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefcalArgumentDefinitionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefcalArgumentDefinitionListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefcalArgumentDefinitionListContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DefcalArgumentDefinitionListContextExt<'input>>{

fn defcalArgumentDefinition_all(&self) ->  Vec<Rc<DefcalArgumentDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn defcalArgumentDefinition(&self, i: usize) -> Option<Rc<DefcalArgumentDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> DefcalArgumentDefinitionListContextAttrs<'input> for DefcalArgumentDefinitionListContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defcalArgumentDefinitionList(&mut self,)
	-> Result<Rc<DefcalArgumentDefinitionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefcalArgumentDefinitionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_defcalArgumentDefinitionList);
        let mut _localctx: Rc<DefcalArgumentDefinitionListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule defcalArgumentDefinition*/
			recog.base.set_state(731);
			recog.defcalArgumentDefinition()?;

			recog.base.set_state(736);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(732);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule defcalArgumentDefinition*/
					recog.base.set_state(733);
					recog.defcalArgumentDefinition()?;

					}
					} 
				}
				recog.base.set_state(738);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
			}
			recog.base.set_state(740);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(739);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defcalOperandList ----------------
pub type DefcalOperandListContextAll<'input> = DefcalOperandListContext<'input>;


pub type DefcalOperandListContext<'input> = BaseParserRuleContext<'input,DefcalOperandListContextExt<'input>>;

#[derive(Clone)]
pub struct DefcalOperandListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for DefcalOperandListContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for DefcalOperandListContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defcalOperandList(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_defcalOperandList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for DefcalOperandListContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_defcalOperandList(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefcalOperandListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defcalOperandList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defcalOperandList }
}
antlr_rust::tid!{DefcalOperandListContextExt<'a>}

impl<'input> DefcalOperandListContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefcalOperandListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefcalOperandListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefcalOperandListContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<DefcalOperandListContextExt<'input>>{

fn defcalOperand_all(&self) ->  Vec<Rc<DefcalOperandContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn defcalOperand(&self, i: usize) -> Option<Rc<DefcalOperandContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> DefcalOperandListContextAttrs<'input> for DefcalOperandListContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defcalOperandList(&mut self,)
	-> Result<Rc<DefcalOperandListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefcalOperandListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_defcalOperandList);
        let mut _localctx: Rc<DefcalOperandListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule defcalOperand*/
			recog.base.set_state(742);
			recog.defcalOperand()?;

			recog.base.set_state(747);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(84,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(743);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule defcalOperand*/
					recog.base.set_state(744);
					recog.defcalOperand()?;

					}
					} 
				}
				recog.base.set_state(749);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(84,&mut recog.base)?;
			}
			recog.base.set_state(751);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(750);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expressionList ----------------
pub type ExpressionListContextAll<'input> = ExpressionListContext<'input>;


pub type ExpressionListContext<'input> = BaseParserRuleContext<'input,ExpressionListContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ExpressionListContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ExpressionListContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expressionList(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_expressionList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ExpressionListContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_expressionList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionList }
}
antlr_rust::tid!{ExpressionListContextExt<'a>}

impl<'input> ExpressionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionListContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ExpressionListContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ExpressionListContextAttrs<'input> for ExpressionListContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionList(&mut self,)
	-> Result<Rc<ExpressionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_expressionList);
        let mut _localctx: Rc<ExpressionListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(753);
			recog.expression_rec(0)?;

			recog.base.set_state(758);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(86,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(754);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(755);
					recog.expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(760);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(86,&mut recog.base)?;
			}
			recog.base.set_state(762);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(761);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifierList ----------------
pub type IdentifierListContextAll<'input> = IdentifierListContext<'input>;


pub type IdentifierListContext<'input> = BaseParserRuleContext<'input,IdentifierListContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for IdentifierListContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for IdentifierListContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifierList(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_identifierList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for IdentifierListContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_identifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierList }
}
antlr_rust::tid!{IdentifierListContextExt<'a>}

impl<'input> IdentifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierListContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<IdentifierListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> IdentifierListContextAttrs<'input> for IdentifierListContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierList(&mut self,)
	-> Result<Rc<IdentifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_identifierList);
        let mut _localctx: Rc<IdentifierListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(764);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(769);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(765);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					recog.base.set_state(766);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(771);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			}
			recog.base.set_state(773);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(772);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- gateOperandList ----------------
pub type GateOperandListContextAll<'input> = GateOperandListContext<'input>;


pub type GateOperandListContext<'input> = BaseParserRuleContext<'input,GateOperandListContextExt<'input>>;

#[derive(Clone)]
pub struct GateOperandListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for GateOperandListContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for GateOperandListContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_gateOperandList(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_gateOperandList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for GateOperandListContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_gateOperandList(self);
	}
}

impl<'input> CustomRuleContext<'input> for GateOperandListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gateOperandList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gateOperandList }
}
antlr_rust::tid!{GateOperandListContextExt<'a>}

impl<'input> GateOperandListContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GateOperandListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GateOperandListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GateOperandListContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<GateOperandListContextExt<'input>>{

fn gateOperand_all(&self) ->  Vec<Rc<GateOperandContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn gateOperand(&self, i: usize) -> Option<Rc<GateOperandContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> GateOperandListContextAttrs<'input> for GateOperandListContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn gateOperandList(&mut self,)
	-> Result<Rc<GateOperandListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GateOperandListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_gateOperandList);
        let mut _localctx: Rc<GateOperandListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule gateOperand*/
			recog.base.set_state(775);
			recog.gateOperand()?;

			recog.base.set_state(780);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(776);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule gateOperand*/
					recog.base.set_state(777);
					recog.gateOperand()?;

					}
					} 
				}
				recog.base.set_state(782);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
			}
			recog.base.set_state(784);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(783);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- externArgumentList ----------------
pub type ExternArgumentListContextAll<'input> = ExternArgumentListContext<'input>;


pub type ExternArgumentListContext<'input> = BaseParserRuleContext<'input,ExternArgumentListContextExt<'input>>;

#[derive(Clone)]
pub struct ExternArgumentListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> qasm3ParserContext<'input> for ExternArgumentListContext<'input>{}

impl<'input,'a> Listenable<dyn qasm3ParserListener<'input> + 'a> for ExternArgumentListContext<'input>{
		fn enter(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_externArgumentList(self);
		}
		fn exit(&self,listener: &mut (dyn qasm3ParserListener<'input> + 'a)) {
			listener.exit_externArgumentList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn qasm3ParserVisitor<'input> + 'a> for ExternArgumentListContext<'input>{
	fn accept(&self,visitor: &mut (dyn qasm3ParserVisitor<'input> + 'a)) {
		visitor.visit_externArgumentList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExternArgumentListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = qasm3ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externArgumentList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externArgumentList }
}
antlr_rust::tid!{ExternArgumentListContextExt<'a>}

impl<'input> ExternArgumentListContextExt<'input>{
	fn new(parent: Option<Rc<dyn qasm3ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExternArgumentListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExternArgumentListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExternArgumentListContextAttrs<'input>: qasm3ParserContext<'input> + BorrowMut<ExternArgumentListContextExt<'input>>{

fn externArgument_all(&self) ->  Vec<Rc<ExternArgumentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn externArgument(&self, i: usize) -> Option<Rc<ExternArgumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,qasm3ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,qasm3ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ExternArgumentListContextAttrs<'input> for ExternArgumentListContext<'input>{}

impl<'input, I, H> qasm3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn externArgumentList(&mut self,)
	-> Result<Rc<ExternArgumentListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExternArgumentListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_externArgumentList);
        let mut _localctx: Rc<ExternArgumentListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule externArgument*/
			recog.base.set_state(786);
			recog.externArgument()?;

			recog.base.set_state(791);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(92,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(787);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule externArgument*/
					recog.base.set_state(788);
					recog.externArgument()?;

					}
					} 
				}
				recog.base.set_state(793);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(92,&mut recog.base)?;
			}
			recog.base.set_state(795);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(794);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x6f\u{320}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x03\x02\x05\x02\u{84}\x0a\x02\x03\x02\x07\x02\
	\u{87}\x0a\x02\x0c\x02\x0e\x02\u{8a}\x0b\x02\x03\x02\x03\x02\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x04\x03\x04\x07\x04\u{94}\x0a\x04\x0c\x04\x0e\x04\
	\u{97}\x0b\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\
	\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\
	\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\
	\x04\x03\x04\x03\x04\x05\x04\u{b5}\x0a\x04\x05\x04\u{b7}\x0a\x04\x03\x05\
	\x03\x05\x05\x05\u{bb}\x0a\x05\x03\x06\x03\x06\x07\x06\u{bf}\x0a\x06\x0c\
	\x06\x0e\x06\u{c2}\x0b\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x08\
	\x03\x08\x05\x08\u{cb}\x0a\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{e8}\x0a\x0e\x03\x0e\x03\x0e\x03\x0f\
	\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{f3}\x0a\x0f\x03\
	\x10\x03\x10\x03\x10\x05\x10\u{f8}\x0a\x10\x03\x10\x03\x10\x03\x11\x03\x11\
	\x03\x11\x03\x11\x03\x11\x03\x11\x03\x12\x03\x12\x05\x12\u{104}\x0a\x12\
	\x03\x12\x03\x12\x03\x13\x03\x13\x05\x13\u{10a}\x0a\x13\x03\x13\x03\x13\
	\x03\x14\x03\x14\x03\x14\x05\x14\u{111}\x0a\x14\x03\x14\x03\x14\x03\x15\
	\x07\x15\u{116}\x0a\x15\x0c\x15\x0e\x15\u{119}\x0b\x15\x03\x15\x03\x15\x03\
	\x15\x05\x15\u{11e}\x0a\x15\x03\x15\x05\x15\u{121}\x0a\x15\x03\x15\x05\x15\
	\u{124}\x0a\x15\x03\x15\x03\x15\x03\x15\x03\x15\x07\x15\u{12a}\x0a\x15\x0c\
	\x15\x0e\x15\u{12d}\x0b\x15\x03\x15\x03\x15\x03\x15\x05\x15\u{132}\x0a\x15\
	\x03\x15\x05\x15\u{135}\x0a\x15\x03\x15\x05\x15\u{138}\x0a\x15\x03\x15\x05\
	\x15\u{13b}\x0a\x15\x03\x15\x05\x15\u{13e}\x0a\x15\x03\x16\x03\x16\x03\x16\
	\x05\x16\u{143}\x0a\x16\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\
	\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x19\x03\x19\x05\x19\
	\u{153}\x0a\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{158}\x0a\x19\x03\x19\x03\
	\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1b\x03\
	\x1b\x03\x1b\x05\x1b\u{166}\x0a\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\
	\x1c\x03\x1c\x05\x1c\u{16e}\x0a\x1c\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x03\
	\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{17a}\x0a\x1e\x03\
	\x1e\x03\x1e\x05\x1e\u{17e}\x0a\x1e\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x05\x1f\u{186}\x0a\x1f\x03\x1f\x03\x1f\x05\x1f\u{18a}\x0a\x1f\
	\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x05\x20\u{192}\x0a\x20\
	\x03\x20\x05\x20\u{195}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x21\x03\x21\
	\x03\x21\x03\x21\x05\x21\u{19e}\x0a\x21\x03\x21\x03\x21\x03\x22\x03\x22\
	\x03\x22\x03\x23\x03\x23\x03\x23\x05\x23\u{1a8}\x0a\x23\x03\x23\x03\x23\
	\x03\x24\x03\x24\x03\x24\x03\x24\x05\x24\u{1b0}\x0a\x24\x03\x24\x05\x24\
	\u{1b3}\x0a\x24\x03\x24\x03\x24\x05\x24\u{1b7}\x0a\x24\x03\x24\x03\x24\x05\
	\x24\u{1bb}\x0a\x24\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\x03\x25\x03\
	\x25\x03\x25\x03\x25\x03\x25\x03\x25\x05\x25\u{1c8}\x0a\x25\x03\x25\x03\
	\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\
	\x25\x03\x25\x05\x25\u{1d6}\x0a\x25\x03\x25\x03\x25\x05\x25\u{1da}\x0a\x25\
	\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\
	\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\
	\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\
	\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x07\x25\
	\u{1ff}\x0a\x25\x0c\x25\x0e\x25\u{202}\x0b\x25\x03\x26\x03\x26\x03\x26\x07\
	\x26\u{207}\x0a\x26\x0c\x26\x0e\x26\u{20a}\x0b\x26\x03\x27\x03\x27\x03\x27\
	\x05\x27\u{20f}\x0a\x27\x03\x28\x03\x28\x03\x28\x03\x29\x05\x29\u{215}\x0a\
	\x29\x03\x29\x03\x29\x05\x29\u{219}\x0a\x29\x03\x29\x03\x29\x05\x29\u{21d}\
	\x0a\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x07\x2a\u{223}\x0a\x2a\x0c\x2a\
	\x0e\x2a\u{226}\x0b\x2a\x03\x2a\x05\x2a\u{229}\x0a\x2a\x03\x2a\x03\x2a\x03\
	\x2b\x03\x2b\x03\x2b\x05\x2b\u{230}\x0a\x2b\x03\x2b\x03\x2b\x03\x2b\x05\
	\x2b\u{235}\x0a\x2b\x07\x2b\u{237}\x0a\x2b\x0c\x2b\x0e\x2b\u{23a}\x0b\x2b\
	\x03\x2b\x05\x2b\u{23d}\x0a\x2b\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x03\x2c\
	\x03\x2c\x05\x2c\u{245}\x0a\x2c\x03\x2c\x03\x2c\x03\x2c\x05\x2c\u{24a}\x0a\
	\x2c\x07\x2c\u{24c}\x0a\x2c\x0c\x2c\x0e\x2c\u{24f}\x0b\x2c\x03\x2c\x05\x2c\
	\u{252}\x0a\x2c\x05\x2c\u{254}\x0a\x2c\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x07\
	\x2d\u{25a}\x0a\x2d\x0c\x2d\x0e\x2d\u{25d}\x0b\x2d\x03\x2e\x03\x2e\x03\x2e\
	\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\
	\x03\x2f\x03\x2f\x05\x2f\u{26d}\x0a\x2f\x05\x2f\u{26f}\x0a\x2f\x03\x2f\x03\
	\x2f\x03\x30\x03\x30\x05\x30\u{275}\x0a\x30\x03\x30\x03\x30\x05\x30\u{279}\
	\x0a\x30\x03\x30\x03\x30\x05\x30\u{27d}\x0a\x30\x03\x30\x03\x30\x05\x30\
	\u{281}\x0a\x30\x03\x30\x03\x30\x05\x30\u{285}\x0a\x30\x03\x30\x03\x30\x03\
	\x30\x03\x30\x03\x30\x03\x30\x03\x30\x03\x30\x05\x30\u{28f}\x0a\x30\x05\
	\x30\u{291}\x0a\x30\x03\x31\x03\x31\x05\x31\u{295}\x0a\x31\x03\x32\x03\x32\
	\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x33\x03\x33\x03\x33\x03\x33\
	\x03\x33\x03\x33\x03\x33\x03\x33\x03\x33\x05\x33\u{2a7}\x0a\x33\x03\x33\
	\x03\x33\x03\x34\x03\x34\x03\x34\x03\x34\x03\x35\x03\x35\x03\x36\x03\x36\
	\x05\x36\u{2b3}\x0a\x36\x03\x37\x03\x37\x03\x38\x03\x38\x05\x38\u{2b9}\x0a\
	\x38\x03\x39\x03\x39\x03\x39\x03\x39\x05\x39\u{2bf}\x0a\x39\x05\x39\u{2c1}\
	\x0a\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\
	\x03\x3a\x05\x3a\u{2cc}\x0a\x3a\x03\x3a\x03\x3a\x03\x3a\x05\x3a\u{2d1}\x0a\
	\x3a\x03\x3b\x03\x3b\x03\x3b\x07\x3b\u{2d6}\x0a\x3b\x0c\x3b\x0e\x3b\u{2d9}\
	\x0b\x3b\x03\x3b\x05\x3b\u{2dc}\x0a\x3b\x03\x3c\x03\x3c\x03\x3c\x07\x3c\
	\u{2e1}\x0a\x3c\x0c\x3c\x0e\x3c\u{2e4}\x0b\x3c\x03\x3c\x05\x3c\u{2e7}\x0a\
	\x3c\x03\x3d\x03\x3d\x03\x3d\x07\x3d\u{2ec}\x0a\x3d\x0c\x3d\x0e\x3d\u{2ef}\
	\x0b\x3d\x03\x3d\x05\x3d\u{2f2}\x0a\x3d\x03\x3e\x03\x3e\x03\x3e\x07\x3e\
	\u{2f7}\x0a\x3e\x0c\x3e\x0e\x3e\u{2fa}\x0b\x3e\x03\x3e\x05\x3e\u{2fd}\x0a\
	\x3e\x03\x3f\x03\x3f\x03\x3f\x07\x3f\u{302}\x0a\x3f\x0c\x3f\x0e\x3f\u{305}\
	\x0b\x3f\x03\x3f\x05\x3f\u{308}\x0a\x3f\x03\x40\x03\x40\x03\x40\x07\x40\
	\u{30d}\x0a\x40\x0c\x40\x0e\x40\u{310}\x0b\x40\x03\x40\x05\x40\u{313}\x0a\
	\x40\x03\x41\x03\x41\x03\x41\x07\x41\u{318}\x0a\x41\x0c\x41\x0e\x41\u{31b}\
	\x0b\x41\x03\x41\x05\x41\u{31e}\x0a\x41\x03\x41\x02\x03\x48\x42\x02\x04\
	\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\
	\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\
	\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\
	\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\x02\x0d\x03\x02\x18\x19\x04\x02\x1d\x1d\
	\x1f\x1f\x04\x02\x41\x41\x53\x53\x04\x02\x45\x45\x50\x51\x04\x02\x36\x36\
	\x57\x60\x04\x02\x46\x46\x48\x49\x04\x02\x43\x43\x45\x45\x03\x02\x2e\x2f\
	\x03\x02\x1b\x1c\x04\x02\x32\x34\x5c\x5c\x03\x02\x5c\x5d\x02\u{373}\x02\
	\u{83}\x03\x02\x02\x02\x04\u{8d}\x03\x02\x02\x02\x06\u{b6}\x03\x02\x02\x02\
	\x08\u{b8}\x03\x02\x02\x02\x0a\u{bc}\x03\x02\x02\x02\x0c\u{c5}\x03\x02\x02\
	\x02\x0e\u{ca}\x03\x02\x02\x02\x10\u{cc}\x03\x02\x02\x02\x12\u{d0}\x03\x02\
	\x02\x02\x14\u{d4}\x03\x02\x02\x02\x16\u{d7}\x03\x02\x02\x02\x18\u{da}\x03\
	\x02\x02\x02\x1a\u{dd}\x03\x02\x02\x02\x1c\u{eb}\x03\x02\x02\x02\x1e\u{f4}\
	\x03\x02\x02\x02\x20\u{fb}\x03\x02\x02\x02\x22\u{101}\x03\x02\x02\x02\x24\
	\u{107}\x03\x02\x02\x02\x26\u{10d}\x03\x02\x02\x02\x28\u{13d}\x03\x02\x02\
	\x02\x2a\u{13f}\x03\x02\x02\x02\x2c\u{146}\x03\x02\x02\x02\x2e\u{14a}\x03\
	\x02\x02\x02\x30\u{152}\x03\x02\x02\x02\x32\u{15b}\x03\x02\x02\x02\x34\u{162}\
	\x03\x02\x02\x02\x36\u{16a}\x03\x02\x02\x02\x38\u{171}\x03\x02\x02\x02\x3a\
	\u{175}\x03\x02\x02\x02\x3c\u{181}\x03\x02\x02\x02\x3e\u{18d}\x03\x02\x02\
	\x02\x40\u{199}\x03\x02\x02\x02\x42\u{1a1}\x03\x02\x02\x02\x44\u{1a4}\x03\
	\x02\x02\x02\x46\u{1ab}\x03\x02\x02\x02\x48\u{1d9}\x03\x02\x02\x02\x4a\u{203}\
	\x03\x02\x02\x02\x4c\u{20e}\x03\x02\x02\x02\x4e\u{210}\x03\x02\x02\x02\x50\
	\u{214}\x03\x02\x02\x02\x52\u{21e}\x03\x02\x02\x02\x54\u{22c}\x03\x02\x02\
	\x02\x56\u{240}\x03\x02\x02\x02\x58\u{257}\x03\x02\x02\x02\x5a\u{25e}\x03\
	\x02\x02\x02\x5c\u{26e}\x03\x02\x02\x02\x5e\u{290}\x03\x02\x02\x02\x60\u{292}\
	\x03\x02\x02\x02\x62\u{296}\x03\x02\x02\x02\x64\u{29d}\x03\x02\x02\x02\x66\
	\u{2aa}\x03\x02\x02\x02\x68\u{2ae}\x03\x02\x02\x02\x6a\u{2b2}\x03\x02\x02\
	\x02\x6c\u{2b4}\x03\x02\x02\x02\x6e\u{2b8}\x03\x02\x02\x02\x70\u{2c0}\x03\
	\x02\x02\x02\x72\u{2d0}\x03\x02\x02\x02\x74\u{2d2}\x03\x02\x02\x02\x76\u{2dd}\
	\x03\x02\x02\x02\x78\u{2e8}\x03\x02\x02\x02\x7a\u{2f3}\x03\x02\x02\x02\x7c\
	\u{2fe}\x03\x02\x02\x02\x7e\u{309}\x03\x02\x02\x02\u{80}\u{314}\x03\x02\
	\x02\x02\u{82}\u{84}\x05\x04\x03\x02\u{83}\u{82}\x03\x02\x02\x02\u{83}\u{84}\
	\x03\x02\x02\x02\u{84}\u{88}\x03\x02\x02\x02\u{85}\u{87}\x05\x06\x04\x02\
	\u{86}\u{85}\x03\x02\x02\x02\u{87}\u{8a}\x03\x02\x02\x02\u{88}\u{86}\x03\
	\x02\x02\x02\u{88}\u{89}\x03\x02\x02\x02\u{89}\u{8b}\x03\x02\x02\x02\u{8a}\
	\u{88}\x03\x02\x02\x02\u{8b}\u{8c}\x07\x02\x02\x03\u{8c}\x03\x03\x02\x02\
	\x02\u{8d}\u{8e}\x07\x03\x02\x02\u{8e}\u{8f}\x07\x67\x02\x02\u{8f}\u{90}\
	\x07\x3e\x02\x02\u{90}\x05\x03\x02\x02\x02\u{91}\u{b7}\x05\x0c\x07\x02\u{92}\
	\u{94}\x05\x08\x05\x02\u{93}\u{92}\x03\x02\x02\x02\u{94}\u{97}\x03\x02\x02\
	\x02\u{95}\u{93}\x03\x02\x02\x02\u{95}\u{96}\x03\x02\x02\x02\u{96}\u{b4}\
	\x03\x02\x02\x02\u{97}\u{95}\x03\x02\x02\x02\u{98}\u{b5}\x05\x2e\x18\x02\
	\u{99}\u{b5}\x05\x40\x21\x02\u{9a}\u{b5}\x05\x22\x12\x02\u{9b}\u{b5}\x05\
	\x24\x13\x02\u{9c}\u{b5}\x05\x14\x0b\x02\u{9d}\u{b5}\x05\x44\x23\x02\u{9e}\
	\u{b5}\x05\x10\x09\x02\u{9f}\u{b5}\x05\x30\x19\x02\u{a0}\u{b5}\x05\x32\x1a\
	\x02\u{a1}\u{b5}\x05\x16\x0c\x02\u{a2}\u{b5}\x05\x3a\x1e\x02\u{a3}\u{b5}\
	\x05\x46\x24\x02\u{a4}\u{b5}\x05\x26\x14\x02\u{a5}\u{b5}\x05\x18\x0d\x02\
	\u{a6}\u{b5}\x05\x42\x22\x02\u{a7}\u{b5}\x05\x3c\x1f\x02\u{a8}\u{b5}\x05\
	\x1a\x0e\x02\u{a9}\u{b5}\x05\x28\x15\x02\u{aa}\u{b5}\x05\x3e\x20\x02\u{ab}\
	\u{b5}\x05\x1c\x0f\x02\u{ac}\u{b5}\x05\x12\x0a\x02\u{ad}\u{b5}\x05\x34\x1b\
	\x02\u{ae}\u{b5}\x05\x2a\x16\x02\u{af}\u{b5}\x05\x36\x1c\x02\u{b0}\u{b5}\
	\x05\x38\x1d\x02\u{b1}\u{b5}\x05\x2c\x17\x02\u{b2}\u{b5}\x05\x1e\x10\x02\
	\u{b3}\u{b5}\x05\x20\x11\x02\u{b4}\u{98}\x03\x02\x02\x02\u{b4}\u{99}\x03\
	\x02\x02\x02\u{b4}\u{9a}\x03\x02\x02\x02\u{b4}\u{9b}\x03\x02\x02\x02\u{b4}\
	\u{9c}\x03\x02\x02\x02\u{b4}\u{9d}\x03\x02\x02\x02\u{b4}\u{9e}\x03\x02\x02\
	\x02\u{b4}\u{9f}\x03\x02\x02\x02\u{b4}\u{a0}\x03\x02\x02\x02\u{b4}\u{a1}\
	\x03\x02\x02\x02\u{b4}\u{a2}\x03\x02\x02\x02\u{b4}\u{a3}\x03\x02\x02\x02\
	\u{b4}\u{a4}\x03\x02\x02\x02\u{b4}\u{a5}\x03\x02\x02\x02\u{b4}\u{a6}\x03\
	\x02\x02\x02\u{b4}\u{a7}\x03\x02\x02\x02\u{b4}\u{a8}\x03\x02\x02\x02\u{b4}\
	\u{a9}\x03\x02\x02\x02\u{b4}\u{aa}\x03\x02\x02\x02\u{b4}\u{ab}\x03\x02\x02\
	\x02\u{b4}\u{ac}\x03\x02\x02\x02\u{b4}\u{ad}\x03\x02\x02\x02\u{b4}\u{ae}\
	\x03\x02\x02\x02\u{b4}\u{af}\x03\x02\x02\x02\u{b4}\u{b0}\x03\x02\x02\x02\
	\u{b4}\u{b1}\x03\x02\x02\x02\u{b4}\u{b2}\x03\x02\x02\x02\u{b4}\u{b3}\x03\
	\x02\x02\x02\u{b5}\u{b7}\x03\x02\x02\x02\u{b6}\u{91}\x03\x02\x02\x02\u{b6}\
	\u{95}\x03\x02\x02\x02\u{b7}\x07\x03\x02\x02\x02\u{b8}\u{ba}\x07\x17\x02\
	\x02\u{b9}\u{bb}\x07\x6a\x02\x02\u{ba}\u{b9}\x03\x02\x02\x02\u{ba}\u{bb}\
	\x03\x02\x02\x02\u{bb}\x09\x03\x02\x02\x02\u{bc}\u{c0}\x07\x39\x02\x02\u{bd}\
	\u{bf}\x05\x06\x04\x02\u{be}\u{bd}\x03\x02\x02\x02\u{bf}\u{c2}\x03\x02\x02\
	\x02\u{c0}\u{be}\x03\x02\x02\x02\u{c0}\u{c1}\x03\x02\x02\x02\u{c1}\u{c3}\
	\x03\x02\x02\x02\u{c2}\u{c0}\x03\x02\x02\x02\u{c3}\u{c4}\x07\x3a\x02\x02\
	\u{c4}\x0b\x03\x02\x02\x02\u{c5}\u{c6}\x07\x16\x02\x02\u{c6}\u{c7}\x07\x6a\
	\x02\x02\u{c7}\x0d\x03\x02\x02\x02\u{c8}\u{cb}\x05\x06\x04\x02\u{c9}\u{cb}\
	\x05\x0a\x06\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{ca}\u{c9}\x03\x02\x02\x02\
	\u{cb}\x0f\x03\x02\x02\x02\u{cc}\u{cd}\x07\x05\x02\x02\u{cd}\u{ce}\x07\x61\
	\x02\x02\u{ce}\u{cf}\x07\x3e\x02\x02\u{cf}\x11\x03\x02\x02\x02\u{d0}\u{d1}\
	\x07\x04\x02\x02\u{d1}\u{d2}\x07\x61\x02\x02\u{d2}\u{d3}\x07\x3e\x02\x02\
	\u{d3}\x13\x03\x02\x02\x02\u{d4}\u{d5}\x07\x0d\x02\x02\u{d5}\u{d6}\x07\x3e\
	\x02\x02\u{d6}\x15\x03\x02\x02\x02\u{d7}\u{d8}\x07\x0e\x02\x02\u{d8}\u{d9}\
	\x07\x3e\x02\x02\u{d9}\x17\x03\x02\x02\x02\u{da}\u{db}\x07\x11\x02\x02\u{db}\
	\u{dc}\x07\x3e\x02\x02\u{dc}\x19\x03\x02\x02\x02\u{dd}\u{de}\x07\x13\x02\
	\x02\u{de}\u{df}\x05\x5e\x30\x02\u{df}\u{e0}\x07\x5c\x02\x02\u{e0}\u{e7}\
	\x07\x15\x02\x02\u{e1}\u{e8}\x05\x52\x2a\x02\u{e2}\u{e3}\x07\x37\x02\x02\
	\u{e3}\u{e4}\x05\x50\x29\x02\u{e4}\u{e5}\x07\x38\x02\x02\u{e5}\u{e8}\x03\
	\x02\x02\x02\u{e6}\u{e8}\x07\x5c\x02\x02\u{e7}\u{e1}\x03\x02\x02\x02\u{e7}\
	\u{e2}\x03\x02\x02\x02\u{e7}\u{e6}\x03\x02\x02\x02\u{e8}\u{e9}\x03\x02\x02\
	\x02\u{e9}\u{ea}\x05\x0e\x08\x02\u{ea}\x1b\x03\x02\x02\x02\u{eb}\u{ec}\x07\
	\x0f\x02\x02\u{ec}\u{ed}\x07\x3b\x02\x02\u{ed}\u{ee}\x05\x48\x25\x02\u{ee}\
	\u{ef}\x07\x3c\x02\x02\u{ef}\u{f2}\x05\x0e\x08\x02\u{f0}\u{f1}\x07\x10\x02\
	\x02\u{f1}\u{f3}\x05\x0e\x08\x02\u{f2}\u{f0}\x03\x02\x02\x02\u{f2}\u{f3}\
	\x03\x02\x02\x02\u{f3}\x1d\x03\x02\x02\x02\u{f4}\u{f7}\x07\x12\x02\x02\u{f5}\
	\u{f8}\x05\x48\x25\x02\u{f6}\u{f8}\x05\x4e\x28\x02\u{f7}\u{f5}\x03\x02\x02\
	\x02\u{f7}\u{f6}\x03\x02\x02\x02\u{f7}\u{f8}\x03\x02\x02\x02\u{f8}\u{f9}\
	\x03\x02\x02\x02\u{f9}\u{fa}\x07\x3e\x02\x02\u{fa}\x1f\x03\x02\x02\x02\u{fb}\
	\u{fc}\x07\x14\x02\x02\u{fc}\u{fd}\x07\x3b\x02\x02\u{fd}\u{fe}\x05\x48\x25\
	\x02\u{fe}\u{ff}\x07\x3c\x02\x02\u{ff}\u{100}\x05\x0e\x08\x02\u{100}\x21\
	\x03\x02\x02\x02\u{101}\u{103}\x07\x35\x02\x02\u{102}\u{104}\x05\x7e\x40\
	\x02\u{103}\u{102}\x03\x02\x02\x02\u{103}\u{104}\x03\x02\x02\x02\u{104}\
	\u{105}\x03\x02\x02\x02\u{105}\u{106}\x07\x3e\x02\x02\u{106}\x23\x03\x02\
	\x02\x02\u{107}\u{109}\x07\x0b\x02\x02\u{108}\u{10a}\x05\x66\x34\x02\u{109}\
	\u{108}\x03\x02\x02\x02\u{109}\u{10a}\x03\x02\x02\x02\u{10a}\u{10b}\x03\
	\x02\x02\x02\u{10b}\u{10c}\x05\x0a\x06\x02\u{10c}\x25\x03\x02\x02\x02\u{10d}\
	\u{10e}\x07\x32\x02\x02\u{10e}\u{110}\x05\x66\x34\x02\u{10f}\u{111}\x05\
	\x7e\x40\x02\u{110}\u{10f}\x03\x02\x02\x02\u{110}\u{111}\x03\x02\x02\x02\
	\u{111}\u{112}\x03\x02\x02\x02\u{112}\u{113}\x07\x3e\x02\x02\u{113}\x27\
	\x03\x02\x02\x02\u{114}\u{116}\x05\x5c\x2f\x02\u{115}\u{114}\x03\x02\x02\
	\x02\u{116}\u{119}\x03\x02\x02\x02\u{117}\u{115}\x03\x02\x02\x02\u{117}\
	\u{118}\x03\x02\x02\x02\u{118}\u{11a}\x03\x02\x02\x02\u{119}\u{117}\x03\
	\x02\x02\x02\u{11a}\u{120}\x07\x5c\x02\x02\u{11b}\u{11d}\x07\x3b\x02\x02\
	\u{11c}\u{11e}\x05\x7a\x3e\x02\u{11d}\u{11c}\x03\x02\x02\x02\u{11d}\u{11e}\
	\x03\x02\x02\x02\u{11e}\u{11f}\x03\x02\x02\x02\u{11f}\u{121}\x07\x3c\x02\
	\x02\u{120}\u{11b}\x03\x02\x02\x02\u{120}\u{121}\x03\x02\x02\x02\u{121}\
	\u{123}\x03\x02\x02\x02\u{122}\u{124}\x05\x66\x34\x02\u{123}\u{122}\x03\
	\x02\x02\x02\u{123}\u{124}\x03\x02\x02\x02\u{124}\u{125}\x03\x02\x02\x02\
	\u{125}\u{126}\x05\x7e\x40\x02\u{126}\u{127}\x07\x3e\x02\x02\u{127}\u{13e}\
	\x03\x02\x02\x02\u{128}\u{12a}\x05\x5c\x2f\x02\u{129}\u{128}\x03\x02\x02\
	\x02\u{12a}\u{12d}\x03\x02\x02\x02\u{12b}\u{129}\x03\x02\x02\x02\u{12b}\
	\u{12c}\x03\x02\x02\x02\u{12c}\u{12e}\x03\x02\x02\x02\u{12d}\u{12b}\x03\
	\x02\x02\x02\u{12e}\u{134}\x07\x2b\x02\x02\u{12f}\u{131}\x07\x3b\x02\x02\
	\u{130}\u{132}\x05\x7a\x3e\x02\u{131}\u{130}\x03\x02\x02\x02\u{131}\u{132}\
	\x03\x02\x02\x02\u{132}\u{133}\x03\x02\x02\x02\u{133}\u{135}\x07\x3c\x02\
	\x02\u{134}\u{12f}\x03\x02\x02\x02\u{134}\u{135}\x03\x02\x02\x02\u{135}\
	\u{137}\x03\x02\x02\x02\u{136}\u{138}\x05\x66\x34\x02\u{137}\u{136}\x03\
	\x02\x02\x02\u{137}\u{138}\x03\x02\x02\x02\u{138}\u{13a}\x03\x02\x02\x02\
	\u{139}\u{13b}\x05\x7e\x40\x02\u{13a}\u{139}\x03\x02\x02\x02\u{13a}\u{13b}\
	\x03\x02\x02\x02\u{13b}\u{13c}\x03\x02\x02\x02\u{13c}\u{13e}\x07\x3e\x02\
	\x02\u{13d}\u{117}\x03\x02\x02\x02\u{13d}\u{12b}\x03\x02\x02\x02\u{13e}\
	\x29\x03\x02\x02\x02\u{13f}\u{142}\x05\x4e\x28\x02\u{140}\u{141}\x07\x42\
	\x02\x02\u{141}\u{143}\x05\x58\x2d\x02\u{142}\u{140}\x03\x02\x02\x02\u{142}\
	\u{143}\x03\x02\x02\x02\u{143}\u{144}\x03\x02\x02\x02\u{144}\u{145}\x07\
	\x3e\x02\x02\u{145}\x2b\x03\x02\x02\x02\u{146}\u{147}\x07\x33\x02\x02\u{147}\
	\u{148}\x05\x6e\x38\x02\u{148}\u{149}\x07\x3e\x02\x02\u{149}\x2d\x03\x02\
	\x02\x02\u{14a}\u{14b}\x07\x0c\x02\x02\u{14b}\u{14c}\x07\x5c\x02\x02\u{14c}\
	\u{14d}\x07\x41\x02\x02\u{14d}\u{14e}\x05\x4a\x26\x02\u{14e}\u{14f}\x07\
	\x3e\x02\x02\u{14f}\x2f\x03\x02\x02\x02\u{150}\u{153}\x05\x5e\x30\x02\u{151}\
	\u{153}\x05\x62\x32\x02\u{152}\u{150}\x03\x02\x02\x02\u{152}\u{151}\x03\
	\x02\x02\x02\u{153}\u{154}\x03\x02\x02\x02\u{154}\u{157}\x07\x5c\x02\x02\
	\u{155}\u{156}\x07\x41\x02\x02\u{156}\u{158}\x05\x4c\x27\x02\u{157}\u{155}\
	\x03\x02\x02\x02\u{157}\u{158}\x03\x02\x02\x02\u{158}\u{159}\x03\x02\x02\
	\x02\u{159}\u{15a}\x07\x3e\x02\x02\u{15a}\x31\x03\x02\x02\x02\u{15b}\u{15c}\
	\x07\x1a\x02\x02\u{15c}\u{15d}\x05\x5e\x30\x02\u{15d}\u{15e}\x07\x5c\x02\
	\x02\u{15e}\u{15f}\x07\x41\x02\x02\u{15f}\u{160}\x05\x4c\x27\x02\u{160}\
	\u{161}\x07\x3e\x02\x02\u{161}\x33\x03\x02\x02\x02\u{162}\u{165}\x09\x02\
	\x02\x02\u{163}\u{166}\x05\x5e\x30\x02\u{164}\u{166}\x05\x62\x32\x02\u{165}\
	\u{163}\x03\x02\x02\x02\u{165}\u{164}\x03\x02\x02\x02\u{166}\u{167}\x03\
	\x02\x02\x02\u{167}\u{168}\x07\x5c\x02\x02\u{168}\u{169}\x07\x3e\x02\x02\
	\u{169}\x35\x03\x02\x02\x02\u{16a}\u{16b}\x09\x03\x02\x02\u{16b}\u{16d}\
	\x07\x5c\x02\x02\u{16c}\u{16e}\x05\x66\x34\x02\u{16d}\u{16c}\x03\x02\x02\
	\x02\u{16d}\u{16e}\x03\x02\x02\x02\u{16e}\u{16f}\x03\x02\x02\x02\u{16f}\
	\u{170}\x07\x3e\x02\x02\u{170}\x37\x03\x02\x02\x02\u{171}\u{172}\x05\x60\
	\x31\x02\u{172}\u{173}\x07\x5c\x02\x02\u{173}\u{174}\x07\x3e\x02\x02\u{174}\
	\x39\x03\x02\x02\x02\u{175}\u{176}\x07\x06\x02\x02\u{176}\u{177}\x07\x5c\
	\x02\x02\u{177}\u{179}\x07\x3b\x02\x02\u{178}\u{17a}\x05\x74\x3b\x02\u{179}\
	\u{178}\x03\x02\x02\x02\u{179}\u{17a}\x03\x02\x02\x02\u{17a}\u{17b}\x03\
	\x02\x02\x02\u{17b}\u{17d}\x07\x3c\x02\x02\u{17c}\u{17e}\x05\x5a\x2e\x02\
	\u{17d}\u{17c}\x03\x02\x02\x02\u{17d}\u{17e}\x03\x02\x02\x02\u{17e}\u{17f}\
	\x03\x02\x02\x02\u{17f}\u{180}\x05\x0a\x06\x02\u{180}\x3b\x03\x02\x02\x02\
	\u{181}\u{182}\x07\x0a\x02\x02\u{182}\u{183}\x07\x5c\x02\x02\u{183}\u{185}\
	\x07\x3b\x02\x02\u{184}\u{186}\x05\u{80}\x41\x02\u{185}\u{184}\x03\x02\x02\
	\x02\u{185}\u{186}\x03\x02\x02\x02\u{186}\u{187}\x03\x02\x02\x02\u{187}\
	\u{189}\x07\x3c\x02\x02\u{188}\u{18a}\x05\x5a\x2e\x02\u{189}\u{188}\x03\
	\x02\x02\x02\u{189}\u{18a}\x03\x02\x02\x02\u{18a}\u{18b}\x03\x02\x02\x02\
	\u{18b}\u{18c}\x07\x3e\x02\x02\u{18c}\x3d\x03\x02\x02\x02\u{18d}\u{18e}\
	\x07\x09\x02\x02\u{18e}\u{194}\x07\x5c\x02\x02\u{18f}\u{191}\x07\x3b\x02\
	\x02\u{190}\u{192}\x05\x7c\x3f\x02\u{191}\u{190}\x03\x02\x02\x02\u{191}\
	\u{192}\x03\x02\x02\x02\u{192}\u{193}\x03\x02\x02\x02\u{193}\u{195}\x07\
	\x3c\x02\x02\u{194}\u{18f}\x03\x02\x02\x02\u{194}\u{195}\x03\x02\x02\x02\
	\u{195}\u{196}\x03\x02\x02\x02\u{196}\u{197}\x05\x7c\x3f\x02\u{197}\u{198}\
	\x05\x0a\x06\x02\u{198}\x3f\x03\x02\x02\x02\u{199}\u{19a}\x05\x58\x2d\x02\
	\u{19a}\u{19d}\x09\x04\x02\x02\u{19b}\u{19e}\x05\x48\x25\x02\u{19c}\u{19e}\
	\x05\x4e\x28\x02\u{19d}\u{19b}\x03\x02\x02\x02\u{19d}\u{19c}\x03\x02\x02\
	\x02\u{19e}\u{19f}\x03\x02\x02\x02\u{19f}\u{1a0}\x07\x3e\x02\x02\u{1a0}\
	\x41\x03\x02\x02\x02\u{1a1}\u{1a2}\x05\x48\x25\x02\u{1a2}\u{1a3}\x07\x3e\
	\x02\x02\u{1a3}\x43\x03\x02\x02\x02\u{1a4}\u{1a5}\x07\x07\x02\x02\u{1a5}\
	\u{1a7}\x07\x39\x02\x02\u{1a6}\u{1a8}\x07\x6f\x02\x02\u{1a7}\u{1a6}\x03\
	\x02\x02\x02\u{1a7}\u{1a8}\x03\x02\x02\x02\u{1a8}\u{1a9}\x03\x02\x02\x02\
	\u{1a9}\u{1aa}\x07\x3a\x02\x02\u{1aa}\x45\x03\x02\x02\x02\u{1ab}\u{1ac}\
	\x07\x08\x02\x02\u{1ac}\u{1b2}\x05\x68\x35\x02\u{1ad}\u{1af}\x07\x3b\x02\
	\x02\u{1ae}\u{1b0}\x05\x76\x3c\x02\u{1af}\u{1ae}\x03\x02\x02\x02\u{1af}\
	\u{1b0}\x03\x02\x02\x02\u{1b0}\u{1b1}\x03\x02\x02\x02\u{1b1}\u{1b3}\x07\
	\x3c\x02\x02\u{1b2}\u{1ad}\x03\x02\x02\x02\u{1b2}\u{1b3}\x03\x02\x02\x02\
	\u{1b3}\u{1b4}\x03\x02\x02\x02\u{1b4}\u{1b6}\x05\x78\x3d\x02\u{1b5}\u{1b7}\
	\x05\x5a\x2e\x02\u{1b6}\u{1b5}\x03\x02\x02\x02\u{1b6}\u{1b7}\x03\x02\x02\
	\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\u{1ba}\x07\x39\x02\x02\u{1b9}\
	\u{1bb}\x07\x6f\x02\x02\u{1ba}\u{1b9}\x03\x02\x02\x02\u{1ba}\u{1bb}\x03\
	\x02\x02\x02\u{1bb}\u{1bc}\x03\x02\x02\x02\u{1bc}\u{1bd}\x07\x3a\x02\x02\
	\u{1bd}\x47\x03\x02\x02\x02\u{1be}\u{1bf}\x08\x25\x01\x02\u{1bf}\u{1c0}\
	\x07\x3b\x02\x02\u{1c0}\u{1c1}\x05\x48\x25\x02\u{1c1}\u{1c2}\x07\x3c\x02\
	\x02\u{1c2}\u{1da}\x03\x02\x02\x02\u{1c3}\u{1c4}\x09\x05\x02\x02\u{1c4}\
	\u{1da}\x05\x48\x25\x11\u{1c5}\u{1c8}\x05\x5e\x30\x02\u{1c6}\u{1c8}\x05\
	\x62\x32\x02\u{1c7}\u{1c5}\x03\x02\x02\x02\u{1c7}\u{1c6}\x03\x02\x02\x02\
	\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1ca}\x07\x3b\x02\x02\u{1ca}\u{1cb}\
	\x05\x48\x25\x02\u{1cb}\u{1cc}\x07\x3c\x02\x02\u{1cc}\u{1da}\x03\x02\x02\
	\x02\u{1cd}\u{1ce}\x07\x31\x02\x02\u{1ce}\u{1cf}\x07\x3b\x02\x02\u{1cf}\
	\u{1d0}\x05\x0a\x06\x02\u{1d0}\u{1d1}\x07\x3c\x02\x02\u{1d1}\u{1da}\x03\
	\x02\x02\x02\u{1d2}\u{1d3}\x07\x5c\x02\x02\u{1d3}\u{1d5}\x07\x3b\x02\x02\
	\u{1d4}\u{1d6}\x05\x7a\x3e\x02\u{1d5}\u{1d4}\x03\x02\x02\x02\u{1d5}\u{1d6}\
	\x03\x02\x02\x02\u{1d6}\u{1d7}\x03\x02\x02\x02\u{1d7}\u{1da}\x07\x3c\x02\
	\x02\u{1d8}\u{1da}\x09\x06\x02\x02\u{1d9}\u{1be}\x03\x02\x02\x02\u{1d9}\
	\u{1c3}\x03\x02\x02\x02\u{1d9}\u{1c7}\x03\x02\x02\x02\u{1d9}\u{1cd}\x03\
	\x02\x02\x02\u{1d9}\u{1d2}\x03\x02\x02\x02\u{1d9}\u{1d8}\x03\x02\x02\x02\
	\u{1da}\u{200}\x03\x02\x02\x02\u{1db}\u{1dc}\x0c\x12\x02\x02\u{1dc}\u{1dd}\
	\x07\x47\x02\x02\u{1dd}\u{1ff}\x05\x48\x25\x12\u{1de}\u{1df}\x0c\x10\x02\
	\x02\u{1df}\u{1e0}\x09\x07\x02\x02\u{1e0}\u{1ff}\x05\x48\x25\x11\u{1e1}\
	\u{1e2}\x0c\x0f\x02\x02\u{1e2}\u{1e3}\x09\x08\x02\x02\u{1e3}\u{1ff}\x05\
	\x48\x25\x10\u{1e4}\u{1e5}\x0c\x0e\x02\x02\u{1e5}\u{1e6}\x07\x55\x02\x02\
	\u{1e6}\u{1ff}\x05\x48\x25\x0f\u{1e7}\u{1e8}\x0c\x0d\x02\x02\u{1e8}\u{1e9}\
	\x07\x54\x02\x02\u{1e9}\u{1ff}\x05\x48\x25\x0e\u{1ea}\u{1eb}\x0c\x0c\x02\
	\x02\u{1eb}\u{1ec}\x07\x52\x02\x02\u{1ec}\u{1ff}\x05\x48\x25\x0d\u{1ed}\
	\u{1ee}\x0c\x0b\x02\x02\u{1ee}\u{1ef}\x07\x4c\x02\x02\u{1ef}\u{1ff}\x05\
	\x48\x25\x0c\u{1f0}\u{1f1}\x0c\x0a\x02\x02\u{1f1}\u{1f2}\x07\x4e\x02\x02\
	\u{1f2}\u{1ff}\x05\x48\x25\x0b\u{1f3}\u{1f4}\x0c\x09\x02\x02\u{1f4}\u{1f5}\
	\x07\x4a\x02\x02\u{1f5}\u{1ff}\x05\x48\x25\x0a\u{1f6}\u{1f7}\x0c\x08\x02\
	\x02\u{1f7}\u{1f8}\x07\x4d\x02\x02\u{1f8}\u{1ff}\x05\x48\x25\x09\u{1f9}\
	\u{1fa}\x0c\x07\x02\x02\u{1fa}\u{1fb}\x07\x4b\x02\x02\u{1fb}\u{1ff}\x05\
	\x48\x25\x08\u{1fc}\u{1fd}\x0c\x13\x02\x02\u{1fd}\u{1ff}\x05\x56\x2c\x02\
	\u{1fe}\u{1db}\x03\x02\x02\x02\u{1fe}\u{1de}\x03\x02\x02\x02\u{1fe}\u{1e1}\
	\x03\x02\x02\x02\u{1fe}\u{1e4}\x03\x02\x02\x02\u{1fe}\u{1e7}\x03\x02\x02\
	\x02\u{1fe}\u{1ea}\x03\x02\x02\x02\u{1fe}\u{1ed}\x03\x02\x02\x02\u{1fe}\
	\u{1f0}\x03\x02\x02\x02\u{1fe}\u{1f3}\x03\x02\x02\x02\u{1fe}\u{1f6}\x03\
	\x02\x02\x02\u{1fe}\u{1f9}\x03\x02\x02\x02\u{1fe}\u{1fc}\x03\x02\x02\x02\
	\u{1ff}\u{202}\x03\x02\x02\x02\u{200}\u{1fe}\x03\x02\x02\x02\u{200}\u{201}\
	\x03\x02\x02\x02\u{201}\x49\x03\x02\x02\x02\u{202}\u{200}\x03\x02\x02\x02\
	\u{203}\u{208}\x05\x48\x25\x02\u{204}\u{205}\x07\x44\x02\x02\u{205}\u{207}\
	\x05\x48\x25\x02\u{206}\u{204}\x03\x02\x02\x02\u{207}\u{20a}\x03\x02\x02\
	\x02\u{208}\u{206}\x03\x02\x02\x02\u{208}\u{209}\x03\x02\x02\x02\u{209}\
	\x4b\x03\x02\x02\x02\u{20a}\u{208}\x03\x02\x02\x02\u{20b}\u{20f}\x05\x54\
	\x2b\x02\u{20c}\u{20f}\x05\x48\x25\x02\u{20d}\u{20f}\x05\x4e\x28\x02\u{20e}\
	\u{20b}\x03\x02\x02\x02\u{20e}\u{20c}\x03\x02\x02\x02\u{20e}\u{20d}\x03\
	\x02\x02\x02\u{20f}\x4d\x03\x02\x02\x02\u{210}\u{211}\x07\x34\x02\x02\u{211}\
	\u{212}\x05\x6e\x38\x02\u{212}\x4f\x03\x02\x02\x02\u{213}\u{215}\x05\x48\
	\x25\x02\u{214}\u{213}\x03\x02\x02\x02\u{214}\u{215}\x03\x02\x02\x02\u{215}\
	\u{216}\x03\x02\x02\x02\u{216}\u{218}\x07\x3d\x02\x02\u{217}\u{219}\x05\
	\x48\x25\x02\u{218}\u{217}\x03\x02\x02\x02\u{218}\u{219}\x03\x02\x02\x02\
	\u{219}\u{21c}\x03\x02\x02\x02\u{21a}\u{21b}\x07\x3d\x02\x02\u{21b}\u{21d}\
	\x05\x48\x25\x02\u{21c}\u{21a}\x03\x02\x02\x02\u{21c}\u{21d}\x03\x02\x02\
	\x02\u{21d}\x51\x03\x02\x02\x02\u{21e}\u{21f}\x07\x39\x02\x02\u{21f}\u{224}\
	\x05\x48\x25\x02\u{220}\u{221}\x07\x40\x02\x02\u{221}\u{223}\x05\x48\x25\
	\x02\u{222}\u{220}\x03\x02\x02\x02\u{223}\u{226}\x03\x02\x02\x02\u{224}\
	\u{222}\x03\x02\x02\x02\u{224}\u{225}\x03\x02\x02\x02\u{225}\u{228}\x03\
	\x02\x02\x02\u{226}\u{224}\x03\x02\x02\x02\u{227}\u{229}\x07\x40\x02\x02\
	\u{228}\u{227}\x03\x02\x02\x02\u{228}\u{229}\x03\x02\x02\x02\u{229}\u{22a}\
	\x03\x02\x02\x02\u{22a}\u{22b}\x07\x3a\x02\x02\u{22b}\x53\x03\x02\x02\x02\
	\u{22c}\u{22f}\x07\x39\x02\x02\u{22d}\u{230}\x05\x48\x25\x02\u{22e}\u{230}\
	\x05\x54\x2b\x02\u{22f}\u{22d}\x03\x02\x02\x02\u{22f}\u{22e}\x03\x02\x02\
	\x02\u{230}\u{238}\x03\x02\x02\x02\u{231}\u{234}\x07\x40\x02\x02\u{232}\
	\u{235}\x05\x48\x25\x02\u{233}\u{235}\x05\x54\x2b\x02\u{234}\u{232}\x03\
	\x02\x02\x02\u{234}\u{233}\x03\x02\x02\x02\u{235}\u{237}\x03\x02\x02\x02\
	\u{236}\u{231}\x03\x02\x02\x02\u{237}\u{23a}\x03\x02\x02\x02\u{238}\u{236}\
	\x03\x02\x02\x02\u{238}\u{239}\x03\x02\x02\x02\u{239}\u{23c}\x03\x02\x02\
	\x02\u{23a}\u{238}\x03\x02\x02\x02\u{23b}\u{23d}\x07\x40\x02\x02\u{23c}\
	\u{23b}\x03\x02\x02\x02\u{23c}\u{23d}\x03\x02\x02\x02\u{23d}\u{23e}\x03\
	\x02\x02\x02\u{23e}\u{23f}\x07\x3a\x02\x02\u{23f}\x55\x03\x02\x02\x02\u{240}\
	\u{253}\x07\x37\x02\x02\u{241}\u{254}\x05\x52\x2a\x02\u{242}\u{245}\x05\
	\x48\x25\x02\u{243}\u{245}\x05\x50\x29\x02\u{244}\u{242}\x03\x02\x02\x02\
	\u{244}\u{243}\x03\x02\x02\x02\u{245}\u{24d}\x03\x02\x02\x02\u{246}\u{249}\
	\x07\x40\x02\x02\u{247}\u{24a}\x05\x48\x25\x02\u{248}\u{24a}\x05\x50\x29\
	\x02\u{249}\u{247}\x03\x02\x02\x02\u{249}\u{248}\x03\x02\x02\x02\u{24a}\
	\u{24c}\x03\x02\x02\x02\u{24b}\u{246}\x03\x02\x02\x02\u{24c}\u{24f}\x03\
	\x02\x02\x02\u{24d}\u{24b}\x03\x02\x02\x02\u{24d}\u{24e}\x03\x02\x02\x02\
	\u{24e}\u{251}\x03\x02\x02\x02\u{24f}\u{24d}\x03\x02\x02\x02\u{250}\u{252}\
	\x07\x40\x02\x02\u{251}\u{250}\x03\x02\x02\x02\u{251}\u{252}\x03\x02\x02\
	\x02\u{252}\u{254}\x03\x02\x02\x02\u{253}\u{241}\x03\x02\x02\x02\u{253}\
	\u{244}\x03\x02\x02\x02\u{254}\u{255}\x03\x02\x02\x02\u{255}\u{256}\x07\
	\x38\x02\x02\u{256}\x57\x03\x02\x02\x02\u{257}\u{25b}\x07\x5c\x02\x02\u{258}\
	\u{25a}\x05\x56\x2c\x02\u{259}\u{258}\x03\x02\x02\x02\u{25a}\u{25d}\x03\
	\x02\x02\x02\u{25b}\u{259}\x03\x02\x02\x02\u{25b}\u{25c}\x03\x02\x02\x02\
	\u{25c}\x59\x03\x02\x02\x02\u{25d}\u{25b}\x03\x02\x02\x02\u{25e}\u{25f}\
	\x07\x42\x02\x02\u{25f}\u{260}\x05\x5e\x30\x02\u{260}\x5b\x03\x02\x02\x02\
	\u{261}\u{26f}\x07\x2c\x02\x02\u{262}\u{263}\x07\x2d\x02\x02\u{263}\u{264}\
	\x07\x3b\x02\x02\u{264}\u{265}\x05\x48\x25\x02\u{265}\u{266}\x07\x3c\x02\
	\x02\u{266}\u{26f}\x03\x02\x02\x02\u{267}\u{26c}\x09\x09\x02\x02\u{268}\
	\u{269}\x07\x3b\x02\x02\u{269}\u{26a}\x05\x48\x25\x02\u{26a}\u{26b}\x07\
	\x3c\x02\x02\u{26b}\u{26d}\x03\x02\x02\x02\u{26c}\u{268}\x03\x02\x02\x02\
	\u{26c}\u{26d}\x03\x02\x02\x02\u{26d}\u{26f}\x03\x02\x02\x02\u{26e}\u{261}\
	\x03\x02\x02\x02\u{26e}\u{262}\x03\x02\x02\x02\u{26e}\u{267}\x03\x02\x02\
	\x02\u{26f}\u{270}\x03\x02\x02\x02\u{270}\u{271}\x07\x4f\x02\x02\u{271}\
	\x5d\x03\x02\x02\x02\u{272}\u{274}\x07\x21\x02\x02\u{273}\u{275}\x05\x66\
	\x34\x02\u{274}\u{273}\x03\x02\x02\x02\u{274}\u{275}\x03\x02\x02\x02\u{275}\
	\u{291}\x03\x02\x02\x02\u{276}\u{278}\x07\x22\x02\x02\u{277}\u{279}\x05\
	\x66\x34\x02\u{278}\u{277}\x03\x02\x02\x02\u{278}\u{279}\x03\x02\x02\x02\
	\u{279}\u{291}\x03\x02\x02\x02\u{27a}\u{27c}\x07\x23\x02\x02\u{27b}\u{27d}\
	\x05\x66\x34\x02\u{27c}\u{27b}\x03\x02\x02\x02\u{27c}\u{27d}\x03\x02\x02\
	\x02\u{27d}\u{291}\x03\x02\x02\x02\u{27e}\u{280}\x07\x24\x02\x02\u{27f}\
	\u{281}\x05\x66\x34\x02\u{280}\u{27f}\x03\x02\x02\x02\u{280}\u{281}\x03\
	\x02\x02\x02\u{281}\u{291}\x03\x02\x02\x02\u{282}\u{284}\x07\x25\x02\x02\
	\u{283}\u{285}\x05\x66\x34\x02\u{284}\u{283}\x03\x02\x02\x02\u{284}\u{285}\
	\x03\x02\x02\x02\u{285}\u{291}\x03\x02\x02\x02\u{286}\u{291}\x07\x20\x02\
	\x02\u{287}\u{291}\x07\x29\x02\x02\u{288}\u{291}\x07\x2a\x02\x02\u{289}\
	\u{28e}\x07\x26\x02\x02\u{28a}\u{28b}\x07\x37\x02\x02\u{28b}\u{28c}\x05\
	\x5e\x30\x02\u{28c}\u{28d}\x07\x38\x02\x02\u{28d}\u{28f}\x03\x02\x02\x02\
	\u{28e}\u{28a}\x03\x02\x02\x02\u{28e}\u{28f}\x03\x02\x02\x02\u{28f}\u{291}\
	\x03\x02\x02\x02\u{290}\u{272}\x03\x02\x02\x02\u{290}\u{276}\x03\x02\x02\
	\x02\u{290}\u{27a}\x03\x02\x02\x02\u{290}\u{27e}\x03\x02\x02\x02\u{290}\
	\u{282}\x03\x02\x02\x02\u{290}\u{286}\x03\x02\x02\x02\u{290}\u{287}\x03\
	\x02\x02\x02\u{290}\u{288}\x03\x02\x02\x02\u{290}\u{289}\x03\x02\x02\x02\
	\u{291}\x5f\x03\x02\x02\x02\u{292}\u{294}\x07\x1e\x02\x02\u{293}\u{295}\
	\x05\x66\x34\x02\u{294}\u{293}\x03\x02\x02\x02\u{294}\u{295}\x03\x02\x02\
	\x02\u{295}\x61\x03\x02\x02\x02\u{296}\u{297}\x07\x27\x02\x02\u{297}\u{298}\
	\x07\x37\x02\x02\u{298}\u{299}\x05\x5e\x30\x02\u{299}\u{29a}\x07\x40\x02\
	\x02\u{29a}\u{29b}\x05\x7a\x3e\x02\u{29b}\u{29c}\x07\x38\x02\x02\u{29c}\
	\x63\x03\x02\x02\x02\u{29d}\u{29e}\x09\x0a\x02\x02\u{29e}\u{29f}\x07\x27\
	\x02\x02\u{29f}\u{2a0}\x07\x37\x02\x02\u{2a0}\u{2a1}\x05\x5e\x30\x02\u{2a1}\
	\u{2a6}\x07\x40\x02\x02\u{2a2}\u{2a7}\x05\x7a\x3e\x02\u{2a3}\u{2a4}\x07\
	\x30\x02\x02\u{2a4}\u{2a5}\x07\x41\x02\x02\u{2a5}\u{2a7}\x05\x48\x25\x02\
	\u{2a6}\u{2a2}\x03\x02\x02\x02\u{2a6}\u{2a3}\x03\x02\x02\x02\u{2a7}\u{2a8}\
	\x03\x02\x02\x02\u{2a8}\u{2a9}\x07\x38\x02\x02\u{2a9}\x65\x03\x02\x02\x02\
	\u{2aa}\u{2ab}\x07\x37\x02\x02\u{2ab}\u{2ac}\x05\x48\x25\x02\u{2ac}\u{2ad}\
	\x07\x38\x02\x02\u{2ad}\x67\x03\x02\x02\x02\u{2ae}\u{2af}\x09\x0b\x02\x02\
	\u{2af}\x69\x03\x02\x02\x02\u{2b0}\u{2b3}\x05\x48\x25\x02\u{2b1}\u{2b3}\
	\x05\x72\x3a\x02\u{2b2}\u{2b0}\x03\x02\x02\x02\u{2b2}\u{2b1}\x03\x02\x02\
	\x02\u{2b3}\x6b\x03\x02\x02\x02\u{2b4}\u{2b5}\x09\x0c\x02\x02\u{2b5}\x6d\
	\x03\x02\x02\x02\u{2b6}\u{2b9}\x05\x58\x2d\x02\u{2b7}\u{2b9}\x07\x5d\x02\
	\x02\u{2b8}\u{2b6}\x03\x02\x02\x02\u{2b8}\u{2b7}\x03\x02\x02\x02\u{2b9}\
	\x6f\x03\x02\x02\x02\u{2ba}\u{2c1}\x05\x5e\x30\x02\u{2bb}\u{2c1}\x05\x64\
	\x33\x02\u{2bc}\u{2be}\x07\x1f\x02\x02\u{2bd}\u{2bf}\x05\x66\x34\x02\u{2be}\
	\u{2bd}\x03\x02\x02\x02\u{2be}\u{2bf}\x03\x02\x02\x02\u{2bf}\u{2c1}\x03\
	\x02\x02\x02\u{2c0}\u{2ba}\x03\x02\x02\x02\u{2c0}\u{2bb}\x03\x02\x02\x02\
	\u{2c0}\u{2bc}\x03\x02\x02\x02\u{2c1}\x71\x03\x02\x02\x02\u{2c2}\u{2c3}\
	\x05\x5e\x30\x02\u{2c3}\u{2c4}\x07\x5c\x02\x02\u{2c4}\u{2d1}\x03\x02\x02\
	\x02\u{2c5}\u{2c6}\x05\x60\x31\x02\u{2c6}\u{2c7}\x07\x5c\x02\x02\u{2c7}\
	\u{2d1}\x03\x02\x02\x02\u{2c8}\u{2c9}\x09\x03\x02\x02\u{2c9}\u{2cb}\x07\
	\x5c\x02\x02\u{2ca}\u{2cc}\x05\x66\x34\x02\u{2cb}\u{2ca}\x03\x02\x02\x02\
	\u{2cb}\u{2cc}\x03\x02\x02\x02\u{2cc}\u{2d1}\x03\x02\x02\x02\u{2cd}\u{2ce}\
	\x05\x64\x33\x02\u{2ce}\u{2cf}\x07\x5c\x02\x02\u{2cf}\u{2d1}\x03\x02\x02\
	\x02\u{2d0}\u{2c2}\x03\x02\x02\x02\u{2d0}\u{2c5}\x03\x02\x02\x02\u{2d0}\
	\u{2c8}\x03\x02\x02\x02\u{2d0}\u{2cd}\x03\x02\x02\x02\u{2d1}\x73\x03\x02\
	\x02\x02\u{2d2}\u{2d7}\x05\x72\x3a\x02\u{2d3}\u{2d4}\x07\x40\x02\x02\u{2d4}\
	\u{2d6}\x05\x72\x3a\x02\u{2d5}\u{2d3}\x03\x02\x02\x02\u{2d6}\u{2d9}\x03\
	\x02\x02\x02\u{2d7}\u{2d5}\x03\x02\x02\x02\u{2d7}\u{2d8}\x03\x02\x02\x02\
	\u{2d8}\u{2db}\x03\x02\x02\x02\u{2d9}\u{2d7}\x03\x02\x02\x02\u{2da}\u{2dc}\
	\x07\x40\x02\x02\u{2db}\u{2da}\x03\x02\x02\x02\u{2db}\u{2dc}\x03\x02\x02\
	\x02\u{2dc}\x75\x03\x02\x02\x02\u{2dd}\u{2e2}\x05\x6a\x36\x02\u{2de}\u{2df}\
	\x07\x40\x02\x02\u{2df}\u{2e1}\x05\x6a\x36\x02\u{2e0}\u{2de}\x03\x02\x02\
	\x02\u{2e1}\u{2e4}\x03\x02\x02\x02\u{2e2}\u{2e0}\x03\x02\x02\x02\u{2e2}\
	\u{2e3}\x03\x02\x02\x02\u{2e3}\u{2e6}\x03\x02\x02\x02\u{2e4}\u{2e2}\x03\
	\x02\x02\x02\u{2e5}\u{2e7}\x07\x40\x02\x02\u{2e6}\u{2e5}\x03\x02\x02\x02\
	\u{2e6}\u{2e7}\x03\x02\x02\x02\u{2e7}\x77\x03\x02\x02\x02\u{2e8}\u{2ed}\
	\x05\x6c\x37\x02\u{2e9}\u{2ea}\x07\x40\x02\x02\u{2ea}\u{2ec}\x05\x6c\x37\
	\x02\u{2eb}\u{2e9}\x03\x02\x02\x02\u{2ec}\u{2ef}\x03\x02\x02\x02\u{2ed}\
	\u{2eb}\x03\x02\x02\x02\u{2ed}\u{2ee}\x03\x02\x02\x02\u{2ee}\u{2f1}\x03\
	\x02\x02\x02\u{2ef}\u{2ed}\x03\x02\x02\x02\u{2f0}\u{2f2}\x07\x40\x02\x02\
	\u{2f1}\u{2f0}\x03\x02\x02\x02\u{2f1}\u{2f2}\x03\x02\x02\x02\u{2f2}\x79\
	\x03\x02\x02\x02\u{2f3}\u{2f8}\x05\x48\x25\x02\u{2f4}\u{2f5}\x07\x40\x02\
	\x02\u{2f5}\u{2f7}\x05\x48\x25\x02\u{2f6}\u{2f4}\x03\x02\x02\x02\u{2f7}\
	\u{2fa}\x03\x02\x02\x02\u{2f8}\u{2f6}\x03\x02\x02\x02\u{2f8}\u{2f9}\x03\
	\x02\x02\x02\u{2f9}\u{2fc}\x03\x02\x02\x02\u{2fa}\u{2f8}\x03\x02\x02\x02\
	\u{2fb}\u{2fd}\x07\x40\x02\x02\u{2fc}\u{2fb}\x03\x02\x02\x02\u{2fc}\u{2fd}\
	\x03\x02\x02\x02\u{2fd}\x7b\x03\x02\x02\x02\u{2fe}\u{303}\x07\x5c\x02\x02\
	\u{2ff}\u{300}\x07\x40\x02\x02\u{300}\u{302}\x07\x5c\x02\x02\u{301}\u{2ff}\
	\x03\x02\x02\x02\u{302}\u{305}\x03\x02\x02\x02\u{303}\u{301}\x03\x02\x02\
	\x02\u{303}\u{304}\x03\x02\x02\x02\u{304}\u{307}\x03\x02\x02\x02\u{305}\
	\u{303}\x03\x02\x02\x02\u{306}\u{308}\x07\x40\x02\x02\u{307}\u{306}\x03\
	\x02\x02\x02\u{307}\u{308}\x03\x02\x02\x02\u{308}\x7d\x03\x02\x02\x02\u{309}\
	\u{30e}\x05\x6e\x38\x02\u{30a}\u{30b}\x07\x40\x02\x02\u{30b}\u{30d}\x05\
	\x6e\x38\x02\u{30c}\u{30a}\x03\x02\x02\x02\u{30d}\u{310}\x03\x02\x02\x02\
	\u{30e}\u{30c}\x03\x02\x02\x02\u{30e}\u{30f}\x03\x02\x02\x02\u{30f}\u{312}\
	\x03\x02\x02\x02\u{310}\u{30e}\x03\x02\x02\x02\u{311}\u{313}\x07\x40\x02\
	\x02\u{312}\u{311}\x03\x02\x02\x02\u{312}\u{313}\x03\x02\x02\x02\u{313}\
	\x7f\x03\x02\x02\x02\u{314}\u{319}\x05\x70\x39\x02\u{315}\u{316}\x07\x40\
	\x02\x02\u{316}\u{318}\x05\x70\x39\x02\u{317}\u{315}\x03\x02\x02\x02\u{318}\
	\u{31b}\x03\x02\x02\x02\u{319}\u{317}\x03\x02\x02\x02\u{319}\u{31a}\x03\
	\x02\x02\x02\u{31a}\u{31d}\x03\x02\x02\x02\u{31b}\u{319}\x03\x02\x02\x02\
	\u{31c}\u{31e}\x07\x40\x02\x02\u{31d}\u{31c}\x03\x02\x02\x02\u{31d}\u{31e}\
	\x03\x02\x02\x02\u{31e}\u{81}\x03\x02\x02\x02\x60\u{83}\u{88}\u{95}\u{b4}\
	\u{b6}\u{ba}\u{c0}\u{ca}\u{e7}\u{f2}\u{f7}\u{103}\u{109}\u{110}\u{117}\u{11d}\
	\u{120}\u{123}\u{12b}\u{131}\u{134}\u{137}\u{13a}\u{13d}\u{142}\u{152}\u{157}\
	\u{165}\u{16d}\u{179}\u{17d}\u{185}\u{189}\u{191}\u{194}\u{19d}\u{1a7}\u{1af}\
	\u{1b2}\u{1b6}\u{1ba}\u{1c7}\u{1d5}\u{1d9}\u{1fe}\u{200}\u{208}\u{20e}\u{214}\
	\u{218}\u{21c}\u{224}\u{228}\u{22f}\u{234}\u{238}\u{23c}\u{244}\u{249}\u{24d}\
	\u{251}\u{253}\u{25b}\u{26c}\u{26e}\u{274}\u{278}\u{27c}\u{280}\u{284}\u{28e}\
	\u{290}\u{294}\u{2a6}\u{2b2}\u{2b8}\u{2be}\u{2c0}\u{2cb}\u{2d0}\u{2d7}\u{2db}\
	\u{2e2}\u{2e6}\u{2ed}\u{2f1}\u{2f8}\u{2fc}\u{303}\u{307}\u{30e}\u{312}\u{319}\
	\u{31d}";

