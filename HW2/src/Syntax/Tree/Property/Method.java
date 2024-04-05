package Syntax.Tree.Property;

import Syntax.Exportable;
import Syntax.Token.Ident;
import Syntax.Token.PeekableTokenizer;

import java.util.ArrayList;
import java.util.Iterator;

/**
 * Method
 * <br>
 * {@code fnName({Argument}) retType}
 */
public class Method implements Exportable {
    public Ident fnName;
    public ArrayList<Argument> arguments = new ArrayList<>();
    public Ident retType = new Ident("void");

    public Method(PeekableTokenizer tokenizer) throws Exception {
        this.fnName = tokenizer.next();
        this.fnName.errReserved("fnName");

        if (!tokenizer.next().token.equals("("))
            throw this.fnName.throwError("Error: expect \"(\" at start of method arguments");

        try {
            while (!tokenizer.peek().token.equals(")"))
                if (!tokenizer.peek().token.equals(",")) this.arguments.add(new Argument(tokenizer));
                else tokenizer.next();
        } catch (NullPointerException err) {
            throw this.fnName.throwError("Error: expect \")\" at end of method arguments");
        } finally {
            tokenizer.next();
        }
        // workaround: peek next if "class", peek one if ":"
        if (tokenizer.peek(1) != null)
            if (tokenizer.peek().token.equals("+")||
                    tokenizer.peek().token.equals("-") ||
                    tokenizer.peek().token.equals("class") ||
                    tokenizer.peek(1).token.equals(":") ||
                    tokenizer.peek(1).token.equals("<|--")
            ) return;

        if (!tokenizer.hasNext()) return;

        this.retType = tokenizer.next();
        this.retType.errReserved("retType");
    }

    private String exportArguments() {
        StringBuilder argExport = new StringBuilder();

        Iterator<Argument> arguments = this.arguments.iterator();
        while (arguments.hasNext()) {
            Argument arg = arguments.next();
            argExport.append(arg.export());
            if (arguments.hasNext()) argExport.append(", ");
        }

        return argExport.toString();
    }

    /**
     * get FnName with prefix removed in lower camel format
     *
     * @param prefix prefix to remove
     * @return processed FnName
     */
    private String getCamelFnName(String prefix) {
        String token = fnName.token.substring(prefix.length());
        return Character.toLowerCase(token.charAt(0)) + token.substring(1);
    }

    /**
     * export ItemFn as java
     * <br>
     * see <a href="https://docs.rs/syn/latest/syn/struct.ItemFn.html">syn</a> in Rust
     *
     * @return unidentified string
     */
    private String exportItemFn() throws Exception {
        if(fnName.token.length()>=4&&Character.isUpperCase(fnName.token.charAt(3))) {
            if (fnName.token.startsWith("set")) {
                String assignVal = this.getCamelFnName("set");
                String inputVal;
                try {
                    inputVal = arguments.get(0).argName.token;
                } catch (Exception err) {
                    throw fnName.throwError("Error: expect setter to have at least one argument");
                }
                return String.format("{\n    this.%s = %s;\n}", assignVal, inputVal);
            } else if (fnName.token.startsWith("get"))
                return String.format("{\n    return %s;\n}", this.getCamelFnName("get"));
        }
        switch (retType.token) {
            case "int" -> {
                return "{return 0;}";
            }
            case "String" -> {
                return "{return \"\";}";
            }
            case "boolean" -> {
                return "{return false;}";
            }
            default -> {
                return "{;}";
            }
        }
    }

    @Override
    public String export() throws Exception {
        return String.format("%s %s(%s) %s", retType.token, fnName.token, this.exportArguments(), this.exportItemFn());
    }
}
