#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <ranges>
#include <algorithm>

int main(){

  using namespace std;
  
  const string filename("input.txt");
  
  ifstream infile{filename, ios::in};
  if(!infile.is_open()){
    cerr << "Cannot open " << filename << '\n';
    return 1;
  }

  string line;
  vector<int> list1, list2;

  while(getline(infile, line)){
    auto pos_space = line.find(" ");
    auto str1 = line.substr(0, pos_space);

    while(line[pos_space] == ' '){
      pos_space++;
    }

    auto str2 = line.substr(pos_space);

    list1.push_back(stoi(str1));
    list2.push_back(stoi(str2));
  }

  ranges::sort(list1);
  ranges::sort(list2);

  int diff = 0;
  for(size_t i = 0; i < list1.size(); ++i){
    diff += abs(list1[i] - list2[i]);
  }

  cout << "Difference is : " << diff << '\n';

  int similarity = 0;
  for(int i : list1){
    auto occurences_in_2 = ranges::count(list2, i);
    similarity += i * occurences_in_2;
  }

  cout << "Similarity is : " << similarity << '\n';
}
