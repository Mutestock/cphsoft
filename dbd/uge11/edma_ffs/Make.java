import edma.generator.EdmaGenerator;

public class Make {
    public static void main(String[] args) {
        ClassLoader loader = Make.class.getClassLoader();
        String className = Make.class.getSimpleName()+".class";
        String pathToThisFile = loader.getResource(className).toString();
        pathToThisFile = pathToThisFile.replace("file:/", "");
        String parentDirectory = pathToThisFile.replace("/"+className,"");

        String environmentName = "edma_ffs";
        String edmaSrcDir = parentDirectory + "/edmasrc";
        String javaSrcDir = parentDirectory + "/src";
        String rootPackage = "edma_ffs";

        new EdmaGenerator(
            environmentName,
            edmaSrcDir,
            javaSrcDir,
            rootPackage
        ).make();
    }
}