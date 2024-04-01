import Solvers.ContainSolver;
import Solvers.DuplicateSolver;
import Solvers.PalindromeSolver;

import java.io.BufferedReader;
import java.io.FileReader;

public class RegExp {
    String str1;
    String str2;
    int repeat;

    public RegExp(String[] args) {
        this.str1 = args[1].toLowerCase();
        this.str2 = args[2].toLowerCase();
        this.repeat = Integer.parseInt(args[3]);
    }

    public static void main(String[] args) throws Exception {
        RegExp reg_exp = new RegExp(args);
        BufferedReader reader = new BufferedReader(new FileReader(args[0]));
        String line;
        while ((line = reader.readLine()) != null) reg_exp.solveLine(line);
    }

    public void solveLine(String line) {
        String low = line.toLowerCase();
        boolean task1 = new PalindromeSolver(low).solve();
        boolean task2 = new ContainSolver(low, this.str1).solve();
        boolean task3;
        {
            ContainSolver solver = new ContainSolver(low, this.str2);
            solver.setRepeat(this.repeat);
            task3 = solver.solve();
        }
        ;
        boolean task4 = new DuplicateSolver(low).solve();

        System.out.println((task1 ? "Y" : "N") + "," + (task2 ? "Y" : "N") + "," + (task3 ? "Y" : "N") + "," + (task4 ? "Y" : "N"));
    }
}
