CC = g++
CFLAGS = -Wall -g -std=c++20

TARGET = main

all: $(TARGET)

# $(TARGET): $(TARGET).c
# 	$(CC) $(CFLAGS) -o $(TARGET) $(TARGET).c

$(TARGET): main.o Jxx.h Nodes.o Node.o
	$(CC) $(CFLAGS) -o main main.o Nodes.o Node.o

main.o: main.cpp Nodes.h Node.h
	$(CC) $(CFLAGS) -c main.cpp

Nodes.o: Nodes.cpp Nodes.h Node.h Jxx.h
	$(CC) $(CFLAGS) -c Nodes.cpp

Node.o: Node.cpp Node.h Jxx.h SubLattice.h Spin.h
	$(CC) $(CFLAGS) -c Node.cpp

clean:
	$(RM) $(TARGET) *.o
