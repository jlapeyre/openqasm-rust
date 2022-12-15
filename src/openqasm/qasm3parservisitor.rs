#![allow(nonstandard_style)]
// Generated from ./openqasm/qasm3Parser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::qasm3parser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link qasm3Parser}.
 */
pub trait qasm3ParserVisitor<'input>: ParseTreeVisitor<'input,qasm3ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link qasm3Parser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#version}.
	 * @param ctx the parse tree
	 */
	fn visit_version(&mut self, ctx: &VersionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#scope}.
	 * @param ctx the parse tree
	 */
	fn visit_scope(&mut self, ctx: &ScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#pragma}.
	 * @param ctx the parse tree
	 */
	fn visit_pragma(&mut self, ctx: &PragmaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#statementOrScope}.
	 * @param ctx the parse tree
	 */
	fn visit_statementOrScope(&mut self, ctx: &StatementOrScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#calibrationGrammarStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_calibrationGrammarStatement(&mut self, ctx: &CalibrationGrammarStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#includeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_includeStatement(&mut self, ctx: &IncludeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#breakStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_breakStatement(&mut self, ctx: &BreakStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#continueStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#endStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_endStatement(&mut self, ctx: &EndStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#forStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#ifStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#returnStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#whileStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#barrierStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_barrierStatement(&mut self, ctx: &BarrierStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#boxStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_boxStatement(&mut self, ctx: &BoxStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#delayStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_delayStatement(&mut self, ctx: &DelayStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateCallStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_gateCallStatement(&mut self, ctx: &GateCallStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#measureArrowAssignmentStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_measureArrowAssignmentStatement(&mut self, ctx: &MeasureArrowAssignmentStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#resetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_resetStatement(&mut self, ctx: &ResetStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#aliasDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_aliasDeclarationStatement(&mut self, ctx: &AliasDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#classicalDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_classicalDeclarationStatement(&mut self, ctx: &ClassicalDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#constDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_constDeclarationStatement(&mut self, ctx: &ConstDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#ioDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ioDeclarationStatement(&mut self, ctx: &IoDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#oldStyleDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_oldStyleDeclarationStatement(&mut self, ctx: &OldStyleDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#quantumDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_quantumDeclarationStatement(&mut self, ctx: &QuantumDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_defStatement(&mut self, ctx: &DefStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#externStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_externStatement(&mut self, ctx: &ExternStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_gateStatement(&mut self, ctx: &GateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#assignmentStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentStatement(&mut self, ctx: &AssignmentStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#expressionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#calStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_calStatement(&mut self, ctx: &CalStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_defcalStatement(&mut self, ctx: &DefcalStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bitwiseXorExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseXorExpression(&mut self, ctx: &BitwiseXorExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code additiveExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code durationofExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_durationofExpression(&mut self, ctx: &DurationofExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parenthesisExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_parenthesisExpression(&mut self, ctx: &ParenthesisExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code comparisonExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonExpression(&mut self, ctx: &ComparisonExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalOrExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code castExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code powerExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_powerExpression(&mut self, ctx: &PowerExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bitwiseOrExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code callExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bitshiftExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitshiftExpression(&mut self, ctx: &BitshiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bitwiseAndExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code equalityExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalAndExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code indexExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unaryExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code literalExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#aliasExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_aliasExpression(&mut self, ctx: &AliasExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#declarationExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationExpression(&mut self, ctx: &DeclarationExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#measureExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_measureExpression(&mut self, ctx: &MeasureExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#rangeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_rangeExpression(&mut self, ctx: &RangeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#setExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_setExpression(&mut self, ctx: &SetExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#arrayLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#indexOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_indexOperator(&mut self, ctx: &IndexOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#indexedIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_indexedIdentifier(&mut self, ctx: &IndexedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#returnSignature}.
	 * @param ctx the parse tree
	 */
	fn visit_returnSignature(&mut self, ctx: &ReturnSignatureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_gateModifier(&mut self, ctx: &GateModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_scalarType(&mut self, ctx: &ScalarTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#qubitType}.
	 * @param ctx the parse tree
	 */
	fn visit_qubitType(&mut self, ctx: &QubitTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#arrayType}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#arrayReferenceType}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayReferenceType(&mut self, ctx: &ArrayReferenceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#designator}.
	 * @param ctx the parse tree
	 */
	fn visit_designator(&mut self, ctx: &DesignatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_defcalTarget(&mut self, ctx: &DefcalTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalArgumentDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_defcalArgumentDefinition(&mut self, ctx: &DefcalArgumentDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalOperand}.
	 * @param ctx the parse tree
	 */
	fn visit_defcalOperand(&mut self, ctx: &DefcalOperandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateOperand}.
	 * @param ctx the parse tree
	 */
	fn visit_gateOperand(&mut self, ctx: &GateOperandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#externArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_externArgument(&mut self, ctx: &ExternArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#argumentDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_argumentDefinition(&mut self, ctx: &ArgumentDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#argumentDefinitionList}.
	 * @param ctx the parse tree
	 */
	fn visit_argumentDefinitionList(&mut self, ctx: &ArgumentDefinitionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalArgumentDefinitionList}.
	 * @param ctx the parse tree
	 */
	fn visit_defcalArgumentDefinitionList(&mut self, ctx: &DefcalArgumentDefinitionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalOperandList}.
	 * @param ctx the parse tree
	 */
	fn visit_defcalOperandList(&mut self, ctx: &DefcalOperandListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#expressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateOperandList}.
	 * @param ctx the parse tree
	 */
	fn visit_gateOperandList(&mut self, ctx: &GateOperandListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#externArgumentList}.
	 * @param ctx the parse tree
	 */
	fn visit_externArgumentList(&mut self, ctx: &ExternArgumentListContext<'input>) { self.visit_children(ctx) }

}

pub trait qasm3ParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= qasm3ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link qasm3Parser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#version}.
	 * @param ctx the parse tree
	 */
		fn visit_version(&mut self, ctx: &VersionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#annotation}.
	 * @param ctx the parse tree
	 */
		fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#scope}.
	 * @param ctx the parse tree
	 */
		fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#pragma}.
	 * @param ctx the parse tree
	 */
		fn visit_pragma(&mut self, ctx: &PragmaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#statementOrScope}.
	 * @param ctx the parse tree
	 */
		fn visit_statementOrScope(&mut self, ctx: &StatementOrScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#calibrationGrammarStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_calibrationGrammarStatement(&mut self, ctx: &CalibrationGrammarStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#includeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_includeStatement(&mut self, ctx: &IncludeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#breakStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_breakStatement(&mut self, ctx: &BreakStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#continueStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#endStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_endStatement(&mut self, ctx: &EndStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#forStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#ifStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#returnStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#whileStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#barrierStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_barrierStatement(&mut self, ctx: &BarrierStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#boxStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_boxStatement(&mut self, ctx: &BoxStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#delayStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_delayStatement(&mut self, ctx: &DelayStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateCallStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_gateCallStatement(&mut self, ctx: &GateCallStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#measureArrowAssignmentStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_measureArrowAssignmentStatement(&mut self, ctx: &MeasureArrowAssignmentStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#resetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_resetStatement(&mut self, ctx: &ResetStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#aliasDeclarationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_aliasDeclarationStatement(&mut self, ctx: &AliasDeclarationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#classicalDeclarationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_classicalDeclarationStatement(&mut self, ctx: &ClassicalDeclarationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#constDeclarationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_constDeclarationStatement(&mut self, ctx: &ConstDeclarationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#ioDeclarationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_ioDeclarationStatement(&mut self, ctx: &IoDeclarationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#oldStyleDeclarationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_oldStyleDeclarationStatement(&mut self, ctx: &OldStyleDeclarationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#quantumDeclarationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_quantumDeclarationStatement(&mut self, ctx: &QuantumDeclarationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_defStatement(&mut self, ctx: &DefStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#externStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_externStatement(&mut self, ctx: &ExternStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_gateStatement(&mut self, ctx: &GateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#assignmentStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentStatement(&mut self, ctx: &AssignmentStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#expressionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#calStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_calStatement(&mut self, ctx: &CalStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_defcalStatement(&mut self, ctx: &DefcalStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bitwiseXorExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseXorExpression(&mut self, ctx: &BitwiseXorExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code additiveExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code durationofExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_durationofExpression(&mut self, ctx: &DurationofExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parenthesisExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_parenthesisExpression(&mut self, ctx: &ParenthesisExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code comparisonExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonExpression(&mut self, ctx: &ComparisonExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalOrExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code castExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code powerExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_powerExpression(&mut self, ctx: &PowerExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bitwiseOrExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code callExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bitshiftExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitshiftExpression(&mut self, ctx: &BitshiftExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bitwiseAndExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code equalityExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalAndExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code indexExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unaryExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code literalExpression}
	 * labeled alternative in {@link qasm3Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#aliasExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_aliasExpression(&mut self, ctx: &AliasExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#declarationExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_declarationExpression(&mut self, ctx: &DeclarationExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#measureExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_measureExpression(&mut self, ctx: &MeasureExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#rangeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_rangeExpression(&mut self, ctx: &RangeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#setExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_setExpression(&mut self, ctx: &SetExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#arrayLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#indexOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_indexOperator(&mut self, ctx: &IndexOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#indexedIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_indexedIdentifier(&mut self, ctx: &IndexedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#returnSignature}.
	 * @param ctx the parse tree
	 */
		fn visit_returnSignature(&mut self, ctx: &ReturnSignatureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_gateModifier(&mut self, ctx: &GateModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_scalarType(&mut self, ctx: &ScalarTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#qubitType}.
	 * @param ctx the parse tree
	 */
		fn visit_qubitType(&mut self, ctx: &QubitTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#arrayType}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#arrayReferenceType}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayReferenceType(&mut self, ctx: &ArrayReferenceTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#designator}.
	 * @param ctx the parse tree
	 */
		fn visit_designator(&mut self, ctx: &DesignatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_defcalTarget(&mut self, ctx: &DefcalTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalArgumentDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_defcalArgumentDefinition(&mut self, ctx: &DefcalArgumentDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalOperand}.
	 * @param ctx the parse tree
	 */
		fn visit_defcalOperand(&mut self, ctx: &DefcalOperandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateOperand}.
	 * @param ctx the parse tree
	 */
		fn visit_gateOperand(&mut self, ctx: &GateOperandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#externArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_externArgument(&mut self, ctx: &ExternArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#argumentDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_argumentDefinition(&mut self, ctx: &ArgumentDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#argumentDefinitionList}.
	 * @param ctx the parse tree
	 */
		fn visit_argumentDefinitionList(&mut self, ctx: &ArgumentDefinitionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalArgumentDefinitionList}.
	 * @param ctx the parse tree
	 */
		fn visit_defcalArgumentDefinitionList(&mut self, ctx: &DefcalArgumentDefinitionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#defcalOperandList}.
	 * @param ctx the parse tree
	 */
		fn visit_defcalOperandList(&mut self, ctx: &DefcalOperandListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#expressionList}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#identifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#gateOperandList}.
	 * @param ctx the parse tree
	 */
		fn visit_gateOperandList(&mut self, ctx: &GateOperandListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link qasm3Parser#externArgumentList}.
	 * @param ctx the parse tree
	 */
		fn visit_externArgumentList(&mut self, ctx: &ExternArgumentListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> qasm3ParserVisitor<'input> for T
where
	T: qasm3ParserVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_version(&mut self, ctx: &VersionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_version(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scope(&mut self, ctx: &ScopeContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_scope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pragma(&mut self, ctx: &PragmaContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_pragma(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statementOrScope(&mut self, ctx: &StatementOrScopeContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_statementOrScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_calibrationGrammarStatement(&mut self, ctx: &CalibrationGrammarStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_calibrationGrammarStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_includeStatement(&mut self, ctx: &IncludeStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_includeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_breakStatement(&mut self, ctx: &BreakStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_breakStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_continueStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_endStatement(&mut self, ctx: &EndStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_endStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_forStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_ifStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_returnStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_whileStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_barrierStatement(&mut self, ctx: &BarrierStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_barrierStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_boxStatement(&mut self, ctx: &BoxStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_boxStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_delayStatement(&mut self, ctx: &DelayStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_delayStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gateCallStatement(&mut self, ctx: &GateCallStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_gateCallStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_measureArrowAssignmentStatement(&mut self, ctx: &MeasureArrowAssignmentStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_measureArrowAssignmentStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resetStatement(&mut self, ctx: &ResetStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_resetStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aliasDeclarationStatement(&mut self, ctx: &AliasDeclarationStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_aliasDeclarationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classicalDeclarationStatement(&mut self, ctx: &ClassicalDeclarationStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_classicalDeclarationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constDeclarationStatement(&mut self, ctx: &ConstDeclarationStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_constDeclarationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ioDeclarationStatement(&mut self, ctx: &IoDeclarationStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_ioDeclarationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_oldStyleDeclarationStatement(&mut self, ctx: &OldStyleDeclarationStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_oldStyleDeclarationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_quantumDeclarationStatement(&mut self, ctx: &QuantumDeclarationStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_quantumDeclarationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defStatement(&mut self, ctx: &DefStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_defStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_externStatement(&mut self, ctx: &ExternStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_externStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gateStatement(&mut self, ctx: &GateStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_gateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentStatement(&mut self, ctx: &AssignmentStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_assignmentStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_expressionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_calStatement(&mut self, ctx: &CalStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_calStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defcalStatement(&mut self, ctx: &DefcalStatementContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_defcalStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseXorExpression(&mut self, ctx: &BitwiseXorExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_bitwiseXorExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_additiveExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_durationofExpression(&mut self, ctx: &DurationofExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_durationofExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parenthesisExpression(&mut self, ctx: &ParenthesisExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_parenthesisExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonExpression(&mut self, ctx: &ComparisonExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_comparisonExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_multiplicativeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_logicalOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_castExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powerExpression(&mut self, ctx: &PowerExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_powerExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_bitwiseOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_callExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitshiftExpression(&mut self, ctx: &BitshiftExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_bitshiftExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_bitwiseAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_equalityExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_logicalAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_indexExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_unaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_literalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aliasExpression(&mut self, ctx: &AliasExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_aliasExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarationExpression(&mut self, ctx: &DeclarationExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_declarationExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_measureExpression(&mut self, ctx: &MeasureExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_measureExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rangeExpression(&mut self, ctx: &RangeExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_rangeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setExpression(&mut self, ctx: &SetExpressionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_setExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_arrayLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_indexOperator(&mut self, ctx: &IndexOperatorContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_indexOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_indexedIdentifier(&mut self, ctx: &IndexedIdentifierContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_indexedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnSignature(&mut self, ctx: &ReturnSignatureContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_returnSignature(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gateModifier(&mut self, ctx: &GateModifierContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_gateModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scalarType(&mut self, ctx: &ScalarTypeContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_scalarType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qubitType(&mut self, ctx: &QubitTypeContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_qubitType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_arrayType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayReferenceType(&mut self, ctx: &ArrayReferenceTypeContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_arrayReferenceType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_designator(&mut self, ctx: &DesignatorContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_designator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defcalTarget(&mut self, ctx: &DefcalTargetContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_defcalTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defcalArgumentDefinition(&mut self, ctx: &DefcalArgumentDefinitionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_defcalArgumentDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defcalOperand(&mut self, ctx: &DefcalOperandContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_defcalOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gateOperand(&mut self, ctx: &GateOperandContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_gateOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_externArgument(&mut self, ctx: &ExternArgumentContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_externArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_argumentDefinition(&mut self, ctx: &ArgumentDefinitionContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_argumentDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_argumentDefinitionList(&mut self, ctx: &ArgumentDefinitionListContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_argumentDefinitionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defcalArgumentDefinitionList(&mut self, ctx: &DefcalArgumentDefinitionListContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_defcalArgumentDefinitionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defcalOperandList(&mut self, ctx: &DefcalOperandListContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_defcalOperandList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_expressionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_identifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gateOperandList(&mut self, ctx: &GateOperandListContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_gateOperandList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_externArgumentList(&mut self, ctx: &ExternArgumentListContext<'input>){
		let result = <Self as qasm3ParserVisitorCompat>::visit_externArgumentList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}