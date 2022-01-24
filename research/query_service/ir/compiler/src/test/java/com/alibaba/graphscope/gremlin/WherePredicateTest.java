package com.alibaba.graphscope.gremlin;

import com.alibaba.graphscope.common.intermediate.operator.SelectOp;
import org.apache.tinkerpop.gremlin.process.traversal.P;
import org.apache.tinkerpop.gremlin.process.traversal.Traversal;
import org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.GraphTraversalSource;
import org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.__;
import org.apache.tinkerpop.gremlin.process.traversal.step.filter.WherePredicateStep;
import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.tinkergraph.structure.TinkerFactory;
import com.alibaba.graphscope.gremlin.InterOpCollectionBuilder.StepTransformFactory;
import org.junit.Assert;
import org.junit.Test;

public class WherePredicateTest {
    private Graph graph = TinkerFactory.createModern();
    private GraphTraversalSource g = graph.traversal();

    @Test
    public void g_V_where_eq_a() {
        Traversal traversal = g.V().as("a").out().where(P.eq("a"));
        WherePredicateStep step = (WherePredicateStep) traversal.asAdmin().getEndStep();
        SelectOp selectOp = (SelectOp) StepTransformFactory.WHERE_PREDICATE_STEP.apply(step);
        Assert.assertEquals("@ == @a", selectOp.getPredicate().get().applyArg());
    }

    @Test
    public void g_V_where_a_eq_b() {
        Traversal traversal = g.V().as("a").out().as("b").where("a", P.eq("b"));
        WherePredicateStep step = (WherePredicateStep) traversal.asAdmin().getEndStep();
        SelectOp selectOp = (SelectOp) StepTransformFactory.WHERE_PREDICATE_STEP.apply(step);
        Assert.assertEquals("@a == @b", selectOp.getPredicate().get().applyArg());
    }

    @Test
    public void g_V_where_a_eq_b_or_eq_c() {
        Traversal traversal = g.V().as("a")
                .out().as("b").out().as("c").where("a", P.eq("b").or(P.eq("c")));

        WherePredicateStep step = (WherePredicateStep) traversal.asAdmin().getEndStep();
        SelectOp selectOp = (SelectOp) StepTransformFactory.WHERE_PREDICATE_STEP.apply(step);
        Assert.assertEquals("@a == @b || (@a == @c)", selectOp.getPredicate().get().applyArg());
    }

    @Test
    public void g_V_where_eq_a_age() {
        Traversal traversal = g.V().as("a").out().where(P.eq("a")).by("age");
        WherePredicateStep step = (WherePredicateStep) traversal.asAdmin().getEndStep();
        SelectOp selectOp = (SelectOp) StepTransformFactory.WHERE_PREDICATE_STEP.apply(step);
        Assert.assertEquals("@.age && @a.age && @.age == @a.age", selectOp.getPredicate().get().applyArg());
    }

    @Test
    public void g_V_where_eq_a_values_age() {
        Traversal traversal = g.V().as("a").out().where(P.eq("a")).by(__.values("age"));
        WherePredicateStep step = (WherePredicateStep) traversal.asAdmin().getEndStep();
        SelectOp selectOp = (SelectOp) StepTransformFactory.WHERE_PREDICATE_STEP.apply(step);
        Assert.assertEquals("@.age && @a.age && @.age == @a.age", selectOp.getPredicate().get().applyArg());
    }

    @Test
    public void g_V_where_a_id_eq_b_age() {
        Traversal traversal = g.V().as("a").out().as("b").where("a", P.eq("b")).by("id").by("age");
        WherePredicateStep step = (WherePredicateStep) traversal.asAdmin().getEndStep();
        SelectOp selectOp = (SelectOp) StepTransformFactory.WHERE_PREDICATE_STEP.apply(step);
        Assert.assertEquals("@a.id && @b.age && @a.id == @b.age", selectOp.getPredicate().get().applyArg());
    }

    @Test
    public void g_V_where_a_id_eq_b_age_or_c_id() {
        Traversal traversal = g.V().as("a")
                .out().as("b").out().as("c").where("a", P.eq("b").or(P.eq("c"))).by("id").by("age").by("id");
        WherePredicateStep step = (WherePredicateStep) traversal.asAdmin().getEndStep();
        SelectOp selectOp = (SelectOp) StepTransformFactory.WHERE_PREDICATE_STEP.apply(step);
        Assert.assertEquals("@a.id && @b.age && @a.id == @b.age || (@a.id && @c.id && @a.id == @c.id)",
                selectOp.getPredicate().get().applyArg());
    }
}
