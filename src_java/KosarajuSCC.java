import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;

/**
 * Created by Gennadiy on 02.06.2014.
 */


public class KosarajuSCC {
    Map<Integer, List<Integer>> graph = new HashMap<Integer, List<Integer>>();
    Map<Integer, List<Integer>> reversedGraph = new HashMap<Integer, List<Integer>>();
    List<Boolean> explored;
    List<Integer> finishingTime;                                                        //index correspond to finishing time, element - to node
    int numberOfNodes;
    int counter;
    int sccCounter;
    List<Integer> scc = new ArrayList<Integer>();

    public static void main( String[] args ) {
        List<Integer> result = new KosarajuSCC().find();
        Collections.sort(result);
        for (int i = result.size() - 1; i >= result.size() - 5; i--) {
            System.out.println(result.get(i));
        }
    }


    public KosarajuSCC() {
        try {
            String sourceFile = "/home/nick/Projects/edu/algorithms_i/data/SCC.txt.bak";
            BufferedReader reader = new BufferedReader(new FileReader(sourceFile));
            String[] edgeVertices;
            String nextLine;

            while ((nextLine = reader.readLine()) != null) {
                edgeVertices = nextLine.split(" ");
                int tail = Integer.parseInt(edgeVertices[0]);
                int head = Integer.parseInt(edgeVertices[1]);
                List<Integer> adjacentList;
                if (graph.get(tail) == null) {
                    adjacentList = new ArrayList<Integer>();


                } else {
                    adjacentList = graph.get(tail);
                }

                adjacentList.add(head);
                graph.put(tail, adjacentList);
                if (graph.get(head) == null) {
                    graph.put(head, new ArrayList<Integer>());
                }

                if (reversedGraph.get(head) == null) {
                    adjacentList = new ArrayList<Integer>();
                } else {
                    adjacentList = reversedGraph.get(head);
                }

                adjacentList.add(tail);
                reversedGraph.put(head, adjacentList);
                if (reversedGraph.get(tail) == null) {
                    reversedGraph.put(tail, new ArrayList<Integer>());
                }




            }
        } catch (IOException e) {
            System.out.println("File not found");
        }

        numberOfNodes = graph.keySet().size();


    }

    public List<Integer> find() {
        dfsReverseGraph();

        return scc;
    }

    private void dfsReverseGraph() {
        explored = new ArrayList<Boolean>(Arrays.asList(new Boolean[numberOfNodes + 1]));                                   //size + 1 for simplicity, cause nodes numeration start from 1
        Collections.fill(explored, Boolean.FALSE);

        finishingTime = new ArrayList<Integer>(Arrays.asList(new Integer[numberOfNodes + 1]));
        Collections.fill(finishingTime, 0);
        counter = 0;

        for (int node = numberOfNodes; node >= 1; node-- ) {
            if (!explored.get(node)) {
                dfsRev(reversedGraph, node);
            }
        }
        Collections.fill(explored, Boolean.FALSE);
        sccCounter = -1;

        for (int node = numberOfNodes; node >= 1; node--) {
            int currentNode = finishingTime.get(node);
            if (!explored.get(currentNode)) {
                sccCounter++;
                counter = 0;
                dfsDir(currentNode);
                scc.add(sccCounter, counter);
            }

        }

    }

    private void dfsDir(int currentNode) {
        explored.set(currentNode, true);
        counter++;
        List<Integer> adjacentList = graph.get(currentNode);
        for (Integer adjNode : adjacentList) {
            if (!explored.get(adjNode)) {
                dfsDir(adjNode);
            }
        }

    }

    private void dfsRev(Map<Integer, List<Integer>> graphArg, int nodeArg) {
        explored.set(nodeArg, true);
        List<Integer> adjacentList = graphArg.get(nodeArg);
        for (Integer adjNode : adjacentList) {
            if (!explored.get(adjNode)) {
                dfsRev(graphArg, adjNode);
            }
        }
        counter++;
        finishingTime.set(counter, nodeArg);
    }

}
