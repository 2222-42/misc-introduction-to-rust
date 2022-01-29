require 'ffi'

module AddArray 
    extend FFI::Library
    ffi_lib 'add_array/target/release/libadd_array.so'
    attach_function :add_array, [:uint64, :uint64], :uint64
end


def add_array(n, x)
    a = Array.new(n, 0)
    x.times do
        (0..x - 1).each { |i|
            a[i] += 1
        }
    end

    a.sum
end

puts AddArray::add_array(ARGV[0].to_i, ARGV[1].to_i)