/*
 * Copyright 2020 Alibaba Group Holding Limited.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.alibaba.graphscope.common.intermediate.operator;

import com.alibaba.graphscope.common.IrPlan;
import com.alibaba.graphscope.common.TestUtils;
import com.alibaba.graphscope.common.intermediate.ArgUtils;
import org.javatuples.Pair;
import org.junit.After;
import org.junit.Assert;
import org.junit.Test;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.function.Function;

public class ProjectOpTest {
    private IrPlan irPlan = new IrPlan();

    @Test
    public void projectTagTest() throws IOException {
        ProjectOp op = new ProjectOp();
        List<Pair> projectList = Arrays.asList(Pair.with("@a", ArgUtils.asFfiAlias("a", false)));
        op.setProjectExprWithAlias(new OpArg(projectList, Function.identity()));
        irPlan.appendInterOp(op);
        Assert.assertEquals(TestUtils.readJsonFromResource("project_tag.json"), irPlan.getPlanAsJson());
    }

    @Test
    public void projectKeyTest() throws IOException {
        ProjectOp op = new ProjectOp();
        List<Pair> projectList = Arrays.asList(Pair.with("@.name", ArgUtils.asFfiAlias("name", false)));
        op.setProjectExprWithAlias(new OpArg(projectList, Function.identity()));
        irPlan.appendInterOp(op);
        Assert.assertEquals(TestUtils.readJsonFromResource("project_key.json"), irPlan.getPlanAsJson());
    }

    @Test
    public void projectTagKeyTest() throws IOException {
        ProjectOp op = new ProjectOp();
        List<Pair> projectList = Arrays.asList(Pair.with("@a.name", ArgUtils.asFfiAlias("a_name", false)));
        op.setProjectExprWithAlias(new OpArg(projectList, Function.identity()));
        irPlan.appendInterOp(op);
        Assert.assertEquals(TestUtils.readJsonFromResource("project_tag_key.json"), irPlan.getPlanAsJson());
    }

    @Test
    public void projectTagKeysTest() throws IOException {
        ProjectOp op = new ProjectOp();
        List<Pair> projectList = Arrays.asList(
                Pair.with("@a.name", ArgUtils.asFfiAlias("a_name", false)),
                Pair.with("@b.id", ArgUtils.asFfiAlias("b_id", false)));
        op.setProjectExprWithAlias(new OpArg(projectList, Function.identity()));
        irPlan.appendInterOp(op);
        Assert.assertEquals(TestUtils.readJsonFromResource("project_tag_keys.json"), irPlan.getPlanAsJson());
    }

    @After
    public void after() {
        if (irPlan != null) {
            irPlan.close();
        }
    }
}
