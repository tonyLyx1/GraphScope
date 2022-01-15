package com.alibaba.graphscope.gremlin.result;

import com.alibaba.graphscope.gaia.proto.Common;
import com.alibaba.graphscope.gaia.proto.IrResult;
import com.alibaba.graphscope.gremlin.exception.GremlinResultParserException;
import org.apache.tinkerpop.gremlin.structure.util.detached.DetachedEdge;
import org.apache.tinkerpop.gremlin.structure.util.detached.DetachedVertex;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class ParserUtils {
    private static final Logger logger = LoggerFactory.getLogger(ParserUtils.class);

    public static Object parseElement(IrResult.Element element) {
        switch (element.getInnerCase()) {
            case VERTEX:
                IrResult.Vertex vertex = element.getVertex();
                Map<String, Object> properties = parseProperties(vertex.getPropertiesList());
                return new DetachedVertex(vertex.getId(), vertex.getLabel().getName(), properties);
            case EDGE:
                IrResult.Edge edge = element.getEdge();
                Map<String, Object> edgeProperties = parseProperties(edge.getPropertiesList());
                return new DetachedEdge(edge.getId(), edge.getLabel().getName(), edgeProperties,
                        edge.getSrcId(), edge.getSrcLabel().getName(), edge.getDstId(), edge.getDstLabel().getName());
            case OBJECT:
                return parseCommonValue(element.getObject());
            default:
                throw new GremlinResultParserException(element.getInnerCase() + " is invalid");
        }
    }

    public static List<Object> parseCollection(IrResult.Collection collection) {
        return collection.getCollectionList().stream().map(k -> parseElement(k)).collect(Collectors.toList());
    }

    public static IrResult.Entry getHeadEntry(IrResult.Results results) {
        return results.getRecord().getColumns(0).getEntry();
    }

    private static Object parseCommonValue(Common.Value value) {
        switch (value.getItemCase()) {
            case BOOLEAN:
                return value.getBoolean();
            case I32:
                return value.getI32();
            case I64:
                return value.getI64();
            case F64:
                return value.getF64();
            case STR:
                return value.getStr();
            case PAIR_ARRAY:
                Common.PairArray pairs = value.getPairArray();
                Map pairInMap = new HashMap();
                pairs.getItemList().forEach(pair -> {
                    pairInMap.put(parseCommonValue(pair.getKey()), parseCommonValue(pair.getVal()));
                });
                return pairInMap;
            case STR_ARRAY:
                return value.getStrArray().getItemList();
            case NONE:
                return value.getNone();
            default:
                throw new GremlinResultParserException(value.getItemCase() + " is unsupported yet");

        }
    }

    private static Map<String, Object> parseProperties(List<IrResult.Property> properties) {
        return new HashMap<>();
    }
}
