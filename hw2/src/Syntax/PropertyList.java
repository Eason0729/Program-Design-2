package Syntax;

import Syntax.Tree.Property.Property;

import java.util.ArrayList;

public class PropertyList {
    private final ArrayList<Property> args;

    public PropertyList(ArrayList<Property> args) {
        this.args = args;
    }

    public boolean contain(String typeName, String varName) {
        for (Property arg : args)
            if (arg.isValue() && (arg.value.typeName.token.equals(typeName) && arg.value.varName.token.equals(varName)))
                return true;
        return false;
    }
}
