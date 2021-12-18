@mine = %w(
  20,-10
  20,-9
  20,-8
  20,-7
  20,-6
  20,-5
  21,-10
  21,-9
  21,-8
  21,-7
  21,-6
  21,-5
  22,-10
  22,-9
  22,-8
  22,-7
  22,-6
  22,-5
  23,-10
  23,-9
  23,-8
  23,-7
  23,-6
  23,-5
  24,-10
  24,-9
  24,-8
  24,-7
  24,-6
  24,-5
  25,-10
  25,-9
  25,-8
  25,-7
  25,-6
  25,-5
  26,-10
  26,-9
  26,-8
  26,-7
  26,-6
  26,-5
  27,-10
  27,-9
  27,-8
  27,-7
  27,-6
  27,-5
  28,-10
  28,-9
  28,-8
  28,-7
  28,-6
  28,-5
  29,-10
  29,-9
  29,-8
  29,-7
  29,-6
  29,-5
  30,-10
  30,-9
  30,-8
  30,-7
  30,-6
  30,-5
  6,-4
  6,-3
  6,-2
  12,-4
  12,-3
  12,-2
  13,-4
  13,-3
  13,-2
  14,-4
  14,-3
  14,-2
  15,-4
  15,-3
  15,-2
  6,-1
  9,-2
  9,-1
  10,-2
  10,-1
  11,-2
  11,-1
  7,-1
  7,0
  8,-1
  8,0
  9,0
  6,0
  6,1
  7,1
  8,1
  6,2
  7,2
  6,3
  7,3
  6,4
  7,4
  6,5
  7,5
  6,6
  7,6
  6,7
  7,7
  6,8
  7,8
  6,9
  7,9
)

@mcount = Hash.new(0)
@mine.each { |m| @mcount[m] += 1 }

@theirs = %w(
  23,-10
  25,-9
  27,-5
  29,-6
  22,-6
  21,-7
  9,0
  27,-7
  24,-5
  25,-7
  26,-6
  25,-5
  6,8
  11,-2
  20,-5
  29,-10
  6,3
  28,-7
  8,0
  30,-6
  29,-8
  20,-10
  6,7
  6,4
  6,1
  14,-4
  21,-6
  26,-10
  7,-1
  7,7
  8,-1
  21,-9
  6,2
  20,-7
  30,-10
  14,-3
  20,-8
  13,-2
  7,3
  28,-8
  29,-9
  15,-3
  22,-5
  26,-8
  25,-8
  25,-6
  15,-4
  9,-2
  15,-2
  12,-2
  28,-9
  12,-3
  24,-6
  23,-7
  25,-10
  7,8
  11,-3
  26,-7
  7,1
  23,-9
  6,0
  22,-10
  27,-6
  8,1
  22,-8
  13,-4
  7,6
  28,-6
  11,-4
  12,-4
  26,-9
  7,4
  24,-10
  23,-8
  30,-8
  7,0
  9,-1
  10,-1
  26,-5
  22,-9
  6,5
  7,5
  23,-6
  28,-10
  10,-2
  11,-1
  20,-9
  14,-2
  29,-7
  13,-3
  23,-5
  24,-8
  27,-9
  30,-7
  28,-5
  21,-10
  7,9
  6,6
  21,-5
  27,-10
  7,2
  30,-9
  21,-8
  22,-7
  24,-9
  20,-6
  6,9
  29,-5
  8,-2
  27,-8
  30,-5
  24,-7
)

@tcount = Hash.new(0)
@theirs.each { |m| @tcount[m] += 1 }

@missing = @theirs - @mine
