#include <string>
#include <vector>

struct Mapping {
    int generatedLine;
    int generatedColumn;
    int originalLine;
    int originalColumn;
    int source;
    int name;
};

class SourceMap {
public:
    SourceMap(const std::string *mappings_input, int sources, int names, int line_offset = 0, int column_offset = 0);
    ~SourceMap();
    void addMappings(const std::string *mappings_input, int sources, int names, int line_offset = 0, int column_offset = 0);

private:
    void addMapping(int generatedLine, int *segment, int segmentIndex);
    void readMappings(const std::string& mappings_input, int sources, int names, int line_offset = 0, int column_offset = 0);
    void readRawMappings();
    void cleanupRawMappings();

    // Raw mappings data, used in case no modifications are done
    // This makes sourcemap generation about 10x faster if no changes are needed...
    const std::string* _raw_mappings = nullptr;
    int _raw_sources = 0;
    int _raw_names = 0;

    // Processed mappings, for all kinds of modifying within the sourcemap
    std::vector<Mapping> _parsed_mappings;
    int _parsed_sources = 0;
    int _parsed_names = 0;
};